use std::fs;
use std::path::Path;

use rmcp::model::CallToolResult;
use serde_json::{json, Value};

use crate::cli_bridge;

pub const CODEBASE_SEARCH_ROUTE: &str = "rust-tfidf";

pub fn codebase_search(args: Value) -> Result<CallToolResult, rmcp::ErrorData> {
    let root = match args.get("root") {
        Some(value) => value
            .as_str()
            .filter(|root| !root.trim().is_empty())
            .map(str::to_string)
            .ok_or_else(|| {
                rmcp::ErrorData::invalid_params("root must be a non-empty directory path", None)
            })?,
        None => std::env::var("CODERAG_ROOT").map_err(|_| {
            rmcp::ErrorData::invalid_params(
                "root is required (pass in tool args or set CODERAG_ROOT)",
                None,
            )
        })?,
    };

    let query = args
        .get("query")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|query| !query.is_empty())
        .ok_or_else(|| {
            rmcp::ErrorData::invalid_params("query is required and must not be empty", None)
        })?;

    let limit = match args.get("limit") {
        None => 10,
        Some(value) => match value.as_u64() {
            Some(value) if (1..=100).contains(&value) => value,
            _ => {
                return Err(rmcp::ErrorData::invalid_params(
                    "limit must be an integer between 1 and 100",
                    None,
                ))
            }
        },
    };

    let root_path = Path::new(&root);
    let metadata = fs::metadata(root_path).map_err(|error| {
        rmcp::ErrorData::invalid_params(
            format!("root must be an existing readable directory: {error}"),
            None,
        )
    })?;
    if !metadata.is_dir() {
        return Err(rmcp::ErrorData::invalid_params(
            "root must be a directory",
            None,
        ));
    }

    if let Some(value) = args.get("path_filter") {
        if value
            .as_str()
            .filter(|filter| !filter.trim().is_empty())
            .is_none()
        {
            return Err(rmcp::ErrorData::invalid_params(
                "path_filter must be a non-empty string",
                None,
            ));
        }
    }
    for key in ["file_extensions", "exclude_paths"] {
        if let Some(value) = args.get(key) {
            let Some(values) = value.as_array() else {
                return Err(rmcp::ErrorData::invalid_params(
                    format!("{key} must be an array of non-empty strings"),
                    None,
                ));
            };
            if values.iter().any(|value| {
                value
                    .as_str()
                    .map(|item| item.trim().is_empty())
                    .unwrap_or(true)
            }) {
                return Err(rmcp::ErrorData::invalid_params(
                    format!("{key} must contain only non-empty strings"),
                    None,
                ));
            }
        }
    }

    let include_content = match args.get("include_content") {
        None => true,
        Some(value) => value.as_bool().ok_or_else(|| {
            rmcp::ErrorData::invalid_params("include_content must be a boolean", None)
        })?,
    };

    let _ = cli_bridge::invoke_cli_tool("coderag_index", json!({ "root": root, "mode": "auto" }))?;

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

    let mut search = cli_bridge::invoke_cli_tool("coderag_search", search_args)?;

    if let Some(structured) = search.structured_content.as_mut() {
        structured["tool"] = json!("codebase_search");
        structured["route"] = json!(CODEBASE_SEARCH_ROUTE);
        structured["engine"] = json!(coderag_core::ENGINE_NAME);
    }

    Ok(search)
}

#[cfg(test)]
mod tests {
    use super::codebase_search;
    use serde_json::json;

    #[test]
    fn rejects_wrong_root_type_before_engine_invocation() {
        let error = codebase_search(json!({ "root": 42, "query": "auth" }))
            .expect_err("invalid root should be rejected");

        assert!(error
            .message
            .contains("root must be a non-empty directory path"));
    }

    #[test]
    fn rejects_wrong_filter_shape_before_engine_invocation() {
        let error = codebase_search(json!({
            "root": "/tmp",
            "query": "auth",
            "file_extensions": ".ts"
        }))
        .expect_err("invalid filter should be rejected");

        assert!(error
            .message
            .contains("file_extensions must be an array of non-empty strings"));
    }
}
