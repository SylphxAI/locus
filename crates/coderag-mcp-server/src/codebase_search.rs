use rmcp::model::CallToolResult;
use serde_json::{json, Value};

use crate::cli_bridge;

pub const CODEBASE_SEARCH_ROUTE: &str = "rust-tfidf";

fn nonempty_var(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

/// Tool argument `root`, then launch `--root`, then `LOCUS_ROOT`, then `CODERAG_ROOT`.
fn resolve_search_root(args: &Value) -> Option<String> {
    resolve_ranked_root(
        explicit_root(args),
        crate::launch_args::launch_root(),
        nonempty_var("LOCUS_ROOT"),
        nonempty_var("CODERAG_ROOT"),
    )
}

fn resolve_ranked_root(
    tool_root: Option<String>,
    launch_root: Option<String>,
    locus_root: Option<String>,
    coderag_root: Option<String>,
) -> Option<String> {
    tool_root.or(launch_root).or(locus_root).or(coderag_root)
}

fn explicit_root(args: &Value) -> Option<String> {
    args.get("root")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn refresh_index(root: &str) -> Result<Value, rmcp::ErrorData> {
    let indexed = cli_bridge::invoke_cli_tool(
        "coderag_index",
        json!({ "root": root, "mode": "refresh" }),
    )?;
    Ok(indexed
        .structured_content
        .as_ref()
        .and_then(|body| body.get("index").cloned())
        .unwrap_or_else(|| json!({ "refreshMode": "unknown" })))
}

fn attach_index(result: &mut CallToolResult, index: Value) {
    if let Some(structured) = result.structured_content.as_mut() {
        structured["index"] = index;
    }
}

pub fn codebase_search(args: Value) -> Result<CallToolResult, rmcp::ErrorData> {
    let parsed = coderag_core::parse_codebase_search(&args).map_err(|error| {
        rmcp::ErrorData::invalid_params(error.message, Some(json!({ "code": error.code })))
    })?;
    let root = resolve_search_root(&args).ok_or_else(|| {
        rmcp::ErrorData::invalid_params(
            "root is required (pass it on the tool call, launch with --root, or set LOCUS_ROOT)",
            None,
        )
    })?;
    let index = refresh_index(&root)?;

    let mut search_input = json!({
        "root": root,
        "query": parsed.query,
        "limit": parsed.limit,
        "include_content": parsed.include_content,
    });
    if !parsed.file_extensions.is_empty() {
        search_input["file_extensions"] = json!(parsed.file_extensions);
    }
    if let Some(path_filter) = parsed.path_filter {
        search_input["path_filter"] = json!(path_filter);
    }
    if !parsed.exclude_paths.is_empty() {
        search_input["exclude_paths"] = json!(parsed.exclude_paths);
    }

    let mut search = cli_bridge::invoke_cli_tool("coderag_search", search_input)?;

    if let Some(structured) = search.structured_content.as_mut() {
        structured["tool"] = json!("codebase_search");
        structured["route"] = json!(CODEBASE_SEARCH_ROUTE);
        structured["engine"] = json!(coderag_core::ENGINE_NAME);
    }
    attach_index(&mut search, index);

    Ok(search)
}
pub fn find_related(args: Value) -> Result<CallToolResult, rmcp::ErrorData> {
    let root = resolve_search_root(&args)
        .ok_or_else(|| rmcp::ErrorData::invalid_params("root is required", None))?;
    let path = args
        .get("path")
        .and_then(Value::as_str)
        .ok_or_else(|| rmcp::ErrorData::invalid_params("path is required", None))?;
    let line = args
        .get("line")
        .and_then(Value::as_u64)
        .ok_or_else(|| rmcp::ErrorData::invalid_params("line is required", None))?;
    let limit = args.get("limit").and_then(Value::as_u64).unwrap_or(10);
    let index = refresh_index(&root)?;
    let mut result = cli_bridge::invoke_cli_tool(
        "locus_find_related",
        json!({ "root": root, "path": path, "line": line, "limit": limit }),
    )?;
    if let Some(structured) = result.structured_content.as_mut() {
        structured["tool"] = json!("find_related");
        structured["route"] = json!("rust-related");
        structured["engine"] = json!(coderag_core::ENGINE_NAME);
    }
    attach_index(&mut result, index);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::resolve_search_root;
    use serde_json::json;

    #[test]
    fn rejects_invalid_search_input_before_refresh() {
        let error = super::codebase_search(json!({
            "query": "   ",
            "root": "/nowhere",
            "limit": 0,
        }))
        .expect_err("blank query");
        assert_eq!(error.code, rmcp::model::ErrorCode::INVALID_PARAMS);
        assert_eq!(error.message, "query must not be empty");

        let missing = super::codebase_search(json!({ "root": "/nowhere" })).expect_err("missing");
        assert_eq!(missing.message, "Missing required field: query");

        let extensions = super::codebase_search(json!({
            "query": "auth",
            "file_extensions": [],
        }))
        .expect_err("empty extensions");
        assert_eq!(extensions.code, rmcp::model::ErrorCode::INVALID_PARAMS);
        assert!(extensions.message.contains("file_extensions"));
    }

    #[test]
    fn tool_root_wins_over_launch_root_without_mutating_environment() {
        let before = std::env::var("CODERAG_ROOT");
        let before_locus = std::env::var("LOCUS_ROOT");
        crate::launch_args::set_launch_root("/tmp/locus-launch-root".to_string())
            .expect("launch root can be set once");
        let explicit = resolve_search_root(&json!({
            "root": "/tmp/tool-argument-root",
        }))
        .expect("tool root");
        assert_eq!(explicit, "/tmp/tool-argument-root");
        let launched = resolve_search_root(&json!({})).expect("launch root");
        assert_eq!(launched, "/tmp/locus-launch-root");
        assert_eq!(std::env::var("CODERAG_ROOT").ok(), before.ok());
        assert_eq!(std::env::var("LOCUS_ROOT").ok(), before_locus.ok());
    }

    #[test]
    fn locus_root_beats_coderag_root_and_loses_to_an_explicit_root() {
        assert_eq!(
            super::resolve_ranked_root(None, None, Some("/locus".into()), Some("/coderag".into()))
                .as_deref(),
            Some("/locus")
        );
        assert_eq!(
            super::resolve_ranked_root(
                None,
                Some("/launch".into()),
                Some("/locus".into()),
                Some("/coderag".into()),
            )
            .as_deref(),
            Some("/launch")
        );
        assert_eq!(
            super::resolve_ranked_root(None, None, None, Some("/coderag".into())).as_deref(),
            Some("/coderag")
        );
        assert_eq!(
            super::resolve_ranked_root(
                Some("/tool".into()),
                Some("/launch".into()),
                Some("/locus".into()),
                Some("/coderag".into()),
            )
            .as_deref(),
            Some("/tool")
        );
    }
}
