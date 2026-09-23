use std::path::Path;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

use crate::index::{find_related_index, refresh_index, search_index_with, IndexMode, SearchIndex, SearchOptions};
use crate::search_input::parse_codebase_search;
use crate::store::{load_index, save_index};
use crate::types::ToolEnvelope;

static INDEX: OnceLock<Mutex<Option<SearchIndex>>> = OnceLock::new();

fn index_store() -> &'static Mutex<Option<SearchIndex>> {
    INDEX.get_or_init(|| Mutex::new(None))
}

pub fn handle_tool(tool: &str, input: serde_json::Value) -> ToolEnvelope {
    match tool {
        "coderag_index" => coderag_index(input),
        "coderag_search" => coderag_search(input),
        "locus_find_related" => locus_find_related(input),
        _ => ToolEnvelope::error("UNSUPPORTED_TOOL", &format!("Unknown tool: {tool}")),
    }
}

fn coderag_index(input: serde_json::Value) -> ToolEnvelope {
    let root = match input.get("root").and_then(|v| v.as_str()) {
        Some(value) => value,
        None => return ToolEnvelope::error("INVALID_ROOT", "Missing required field: root"),
    };
    let max_file_bytes = input
        .get("maxFileBytes")
        .and_then(|v| v.as_u64())
        .unwrap_or(1_048_576);
    let mode = input
        .get("mode")
        .and_then(|v| v.as_str())
        .map(IndexMode::parse)
        .unwrap_or(IndexMode::Auto);

    match refresh_index(Path::new(root), max_file_bytes, mode) {
        Ok((index, stats)) => {
            if let Err(message) = save_index(Path::new(root), &index) {
                return ToolEnvelope::error("INDEX_PERSIST_FAILED", &message);
            }
            if let Ok(mut guard) = index_store().lock() {
                *guard = Some(index);
            }
            ToolEnvelope::ok_index(stats)
        }
        Err(message) => ToolEnvelope::error("INDEX_FAILED", &message),
    }
}

fn resolve_index(root: Option<&str>) -> Result<SearchIndex, ToolEnvelope> {
    if let Ok(guard) = index_store().lock() {
        if let Some(index) = guard.clone() {
            if root.is_none() || root == Some(index.root.as_str()) {
                return Ok(index);
            }
        }
    }

    let Some(root) = root else {
        return Err(ToolEnvelope::error(
            "INDEX_NOT_FOUND",
            "No in-memory Rust index exists. Call coderag_index first or pass root to load a snapshot.",
        ));
    };

    match load_index(Path::new(root)) {
        Ok(index) => {
            if let Ok(mut guard) = index_store().lock() {
                *guard = Some(index.clone());
            }
            Ok(index)
        }
        Err(_) => match refresh_index(Path::new(root), 1_048_576, IndexMode::Auto) {
            Ok((index, _)) => {
                let _ = save_index(Path::new(root), &index);
                if let Ok(mut guard) = index_store().lock() {
                    *guard = Some(index.clone());
                }
                Ok(index)
            }
            Err(message) => Err(ToolEnvelope::error("INDEX_FAILED", &message)),
        },
    }
}

fn locus_find_related(input: serde_json::Value) -> ToolEnvelope {
    let started = Instant::now();
    let path = match input.get("path").and_then(|v| v.as_str()) {
        Some(value) => value,
        None => return ToolEnvelope::error("INVALID_PATH", "Missing required field: path"),
    };
    let line = match input.get("line").and_then(|v| v.as_u64()) {
        Some(value) if value > 0 => value as u32,
        _ => return ToolEnvelope::error("INVALID_LINE", "line must be a positive integer"),
    };
    let limit = input.get("limit").and_then(|v| v.as_u64()).unwrap_or(10) as usize;
    let root = input.get("root").and_then(|v| v.as_str());
    let index = match resolve_index(root) {
        Ok(index) => index,
        Err(envelope) => return envelope,
    };
    let results = find_related_index(&index, path, line, limit);
    if results.is_empty() {
        return ToolEnvelope::error("NO_RELATED_CHUNKS", "No related chunks found for that location");
    }
    ToolEnvelope::ok_search(&format!("{path}:{line}"), results, started.elapsed().as_millis() as u64)
}

fn coderag_search(input: serde_json::Value) -> ToolEnvelope {
    let started = Instant::now();
    let parsed = match parse_codebase_search(&input) {
        Ok(parsed) => parsed,
        Err(error) => return ToolEnvelope::error(&error.code, &error.message),
    };
    let root = input.get("root").and_then(|v| v.as_str());
    let index = match resolve_index(root) {
        Ok(index) => index,
        Err(envelope) => return envelope,
    };
    let results = search_index_with(
        &index,
        &parsed.query,
        &SearchOptions {
            limit: parsed.limit,
            include_content: parsed.include_content,
            file_extensions: parsed.file_extensions,
            path_filter: parsed.path_filter,
            exclude_paths: parsed.exclude_paths,
        },
    );
    ToolEnvelope::ok_search(&parsed.query, results, started.elapsed().as_millis() as u64)
}

#[cfg(test)]
mod search_input_tests {
    use super::handle_tool;
    use serde_json::json;

    #[test]
    fn rejects_invalid_search_before_resolve_index() {
        let missing_root = "/definitely/missing/locus-root-xyz";
        let missing = handle_tool("coderag_search", json!({ "root": missing_root }));
        assert_eq!(missing.code.as_deref(), Some("INVALID_QUERY"));
        assert_eq!(missing.message.as_deref(), Some("Missing required field: query"));

        let blank = handle_tool(
            "coderag_search",
            json!({ "query": "   ", "root": missing_root }),
        );
        assert_eq!(blank.code.as_deref(), Some("INVALID_QUERY"));
        assert_eq!(blank.message.as_deref(), Some("query must not be empty"));

        let null_query = handle_tool("coderag_search", json!({ "query": null, "root": missing_root }));
        assert_eq!(null_query.message.as_deref(), Some("query must be a string"));

        let limit = handle_tool(
            "coderag_search",
            json!({ "query": "auth", "limit": 0, "root": missing_root }),
        );
        assert_eq!(limit.code.as_deref(), Some("INVALID_LIMIT"));

        let extensions = handle_tool(
            "coderag_search",
            json!({ "query": "auth", "file_extensions": [], "root": missing_root }),
        );
        assert_eq!(extensions.code.as_deref(), Some("INVALID_FILTER"));
        assert!(extensions.message.unwrap().contains("file_extensions"));
    }
}
