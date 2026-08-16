use rmcp::model::CallToolResult;
use serde_json::{json, Value};

use crate::cli_bridge;

pub const CODEBASE_SEARCH_ROUTE: &str = "rust-tfidf";

pub fn codebase_search(args: Value) -> Result<CallToolResult, rmcp::ErrorData> {
    let root = args
        .get("root")
        .and_then(Value::as_str)
        .map(str::to_string)
        .or_else(|| std::env::var("CODERAG_ROOT").ok())
        .ok_or_else(|| {
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

    let include_content = args
        .get("include_content")
        .and_then(Value::as_bool)
        .unwrap_or(true);

    let _ = cli_bridge::invoke_cli_tool(
        "coderag_index",
        json!({ "root": root, "mode": "auto" }),
    )?;

    let mut search_args = json!({
        "root": root,
        "query": query,
        "limit": limit,
        "include_content": include_content,
    });
    for key in ["file_extensions", "path_filter", "exclude_paths"] {
        if let Some(value) = args.get(key) {
            search_args[key] = value.clone();
        }
    }

    let mut search = cli_bridge::invoke_cli_tool(
        "coderag_search",
        search_args,
    )?;

    if let Some(structured) = search.structured_content.as_mut() {
        structured["tool"] = json!("codebase_search");
        structured["route"] = json!(CODEBASE_SEARCH_ROUTE);
        structured["engine"] = json!(coderag_core::ENGINE_NAME);
    }

    Ok(search)
}
