use rmcp::model::CallToolResult;
use serde_json::{json, Value};

use crate::cli_bridge;

pub const CODEBASE_SEARCH_ROUTE: &str = "rust-tfidf";

/// Launch `--root`, then `CODERAG_ROOT`, then the tool argument.
fn resolve_search_root(args: &Value) -> Option<String> {
    crate::launch_args::launch_root()
        .or_else(|| std::env::var("CODERAG_ROOT").ok())
        .or_else(|| args.get("root").and_then(Value::as_str).map(str::to_string))
}

pub fn codebase_search(args: Value) -> Result<CallToolResult, rmcp::ErrorData> {
    let root = resolve_search_root(&args).ok_or_else(|| {
        rmcp::ErrorData::invalid_params(
            "root is required (pass in tool args or set CODERAG_ROOT)",
            None,
        )
    })?;

    let query = args
        .get("query")
        .and_then(Value::as_str)
        .ok_or_else(|| rmcp::ErrorData::invalid_params("query is required", None))?;

    let limit = args.get("limit").and_then(Value::as_u64).unwrap_or(10);

    let _ = cli_bridge::invoke_cli_tool("coderag_index", json!({ "root": root, "mode": "auto" }))?;

    let mut search = cli_bridge::invoke_cli_tool(
        "coderag_search",
        json!({ "root": root, "query": query, "limit": limit }),
    )?;

    if let Some(structured) = search.structured_content.as_mut() {
        structured["tool"] = json!("codebase_search");
        structured["route"] = json!(CODEBASE_SEARCH_ROUTE);
        structured["engine"] = json!(coderag_core::ENGINE_NAME);
    }

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
    let _ = cli_bridge::invoke_cli_tool("coderag_index", json!({ "root": root, "mode": "auto" }))?;
    let mut result = cli_bridge::invoke_cli_tool(
        "locus_find_related",
        json!({ "root": root, "path": path, "line": line, "limit": limit }),
    )?;
    if let Some(structured) = result.structured_content.as_mut() {
        structured["tool"] = json!("find_related");
        structured["route"] = json!("rust-related");
        structured["engine"] = json!(coderag_core::ENGINE_NAME);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::resolve_search_root;
    use serde_json::json;

    #[test]
    fn configured_launch_root_is_used_without_mutating_process_environment() {
        let before = std::env::var("CODERAG_ROOT");
        crate::launch_args::set_launch_root("/tmp/locus-launch-root".to_string())
            .expect("launch root can be set once");
        let root = resolve_search_root(&json!({
            "root": "/tmp/tool-argument-root",
        }))
        .expect("configured root");
        assert_eq!(root, "/tmp/locus-launch-root");
        assert_eq!(std::env::var("CODERAG_ROOT").ok(), before.ok());
    }
}
