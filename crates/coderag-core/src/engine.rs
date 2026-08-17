use std::path::Path;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

use crate::index::{
    canonical_root, ensure_index_gaps, refresh_index, search_index_with_options, IndexMode,
    SearchIndex, SearchOptions,
};
use crate::store::{load_index, save_index};
use crate::types::ToolEnvelope;

fn index_failure(message: &str) -> ToolEnvelope {
    let code = message
        .split_once(':')
        .map(|(code, _)| code)
        .filter(|code| code.starts_with("INDEX_") || code.starts_with("HASH_"))
        .unwrap_or("INDEX_FAILED");
    ToolEnvelope::error(code, message)
}

static INDEX: OnceLock<Mutex<Option<SearchIndex>>> = OnceLock::new();

fn index_store() -> &'static Mutex<Option<SearchIndex>> {
    INDEX.get_or_init(|| Mutex::new(None))
}

pub fn handle_tool(tool: &str, input: serde_json::Value) -> ToolEnvelope {
    match tool {
        "coderag_index" => coderag_index(input),
        "coderag_search" => coderag_search(input),
        _ => ToolEnvelope::error("UNSUPPORTED_TOOL", &format!("Unknown tool: {tool}")),
    }
}

fn coderag_index(input: serde_json::Value) -> ToolEnvelope {
    let root = match input.get("root").and_then(|v| v.as_str()) {
        Some(value) => value,
        None => return ToolEnvelope::error("INVALID_ROOT", "Missing required field: root"),
    };
    let canonical = match canonical_root(Path::new(root)) {
        Ok(value) => value,
        Err(message) => return ToolEnvelope::error("INVALID_ROOT", &message),
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

    match refresh_index(&canonical, max_file_bytes, mode) {
        Ok((index, stats)) => {
            if let Err(message) = save_index(&canonical, &index) {
                return ToolEnvelope::error("INDEX_PERSIST_FAILED", &message);
            }
            let gaps = index.gaps.clone();
            if let Ok(mut guard) = index_store().lock() {
                *guard = Some(index);
            }
            ToolEnvelope::ok_index_with_gaps(stats, gaps)
        }
        Err(message) => index_failure(&message),
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

    let canonical = canonical_root(Path::new(root))
        .map_err(|message| ToolEnvelope::error("INVALID_ROOT", &message))?;

    match load_index(&canonical) {
        Ok(mut index) => {
            ensure_index_gaps(&mut index);
            if let Ok(mut guard) = index_store().lock() {
                *guard = Some(index.clone());
            }
            Ok(index)
        }
        Err(_) => match refresh_index(&canonical, 1_048_576, IndexMode::Auto) {
            Ok((index, _)) => {
                if let Err(message) = save_index(&canonical, &index) {
                    return Err(ToolEnvelope::error("INDEX_PERSIST_FAILED", &message));
                }
                if let Ok(mut guard) = index_store().lock() {
                    *guard = Some(index.clone());
                }
                Ok(index)
            }
            Err(message) => Err(index_failure(&message)),
        },
    }
}

fn coderag_search(input: serde_json::Value) -> ToolEnvelope {
    let started = Instant::now();
    let query = match input.get("query").and_then(|v| v.as_str()).map(str::trim) {
        Some(value) if !value.is_empty() => value,
        None => return ToolEnvelope::error("INVALID_QUERY", "Missing required field: query"),
        Some(_) => return ToolEnvelope::error("INVALID_QUERY", "Query must not be empty"),
    };
    let limit = match input.get("limit") {
        None => 10,
        Some(value) => match value.as_u64() {
            Some(value) if (1..=100).contains(&value) => value as usize,
            Some(value) => {
                return ToolEnvelope::error(
                    "INVALID_LIMIT",
                    &format!("limit must be between 1 and 100, got {value}"),
                )
            }
            None => {
                return ToolEnvelope::error(
                    "INVALID_LIMIT",
                    "limit must be an integer between 1 and 100",
                )
            }
        },
    };

    let root = input.get("root").and_then(|v| v.as_str());
    let index = match resolve_index(root) {
        Ok(index) => index,
        Err(envelope) => return envelope,
    };

    let options = match serde_json::from_value::<SearchOptions>(input.clone()) {
        Ok(options) => options,
        Err(error) => {
            return ToolEnvelope::error(
                "INVALID_SEARCH_OPTIONS",
                &format!("Invalid search options: {error}"),
            )
        }
    };
    if let Err(message) = options.validate() {
        return ToolEnvelope::error("INVALID_SEARCH_OPTIONS", &message);
    }
    let results = search_index_with_options(&index, query, limit, &options);
    ToolEnvelope::ok_search_with_gaps(
        query,
        results,
        started.elapsed().as_millis() as u64,
        index.gaps.clone(),
    )
}

#[cfg(test)]
mod tests {
    use super::handle_tool;
    use serde_json::json;

    #[test]
    fn search_admission_rejects_empty_query() {
        let envelope = handle_tool("coderag_search", json!({ "root": "/tmp", "query": "   " }));

        assert_eq!(envelope.status, "error");
        assert_eq!(envelope.code.as_deref(), Some("INVALID_QUERY"));
    }

    #[test]
    fn search_admission_rejects_invalid_limit() {
        let envelope = handle_tool(
            "coderag_search",
            json!({ "root": "/tmp", "query": "auth", "limit": 0 }),
        );

        assert_eq!(envelope.status, "error");
        assert_eq!(envelope.code.as_deref(), Some("INVALID_LIMIT"));
    }

    #[test]
    fn search_admission_rejects_non_directory_root() {
        let envelope = handle_tool(
            "coderag_search",
            json!({ "root": "/definitely/not/a/repository", "query": "auth" }),
        );

        assert_eq!(envelope.status, "error");
        assert_eq!(envelope.code.as_deref(), Some("INVALID_ROOT"));
    }

    #[test]
    fn index_reports_specific_read_failure_code() {
        let root = std::env::temp_dir().join(format!(
            "coderag-engine-invalid-utf8-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("temp root");
        std::fs::write(root.join("broken.ts"), [0xff, 0xfe, 0xfd]).expect("invalid source");

        let envelope = handle_tool("coderag_index", json!({ "root": root.to_string_lossy() }));

        assert_eq!(envelope.status, "error");
        assert_eq!(envelope.code.as_deref(), Some("INDEX_READ_FAILED"));
        let _ = std::fs::remove_dir_all(root);
    }
}
