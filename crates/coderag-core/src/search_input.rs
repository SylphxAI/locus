//! Shared codebase_search argument parser.
//!
//! The MCP server and the Rust engine both call this before any index work.

use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchInputError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedCodebaseSearch {
    pub query: String,
    pub limit: usize,
    pub include_content: bool,
    /// Final-extension tokens without a leading dot, case preserved.
    pub file_extensions: Vec<String>,
    /// Slash-normalized substring. `None` means no path filter.
    pub path_filter: Option<String>,
    /// Slash-normalized substrings. Empty means no exclusions.
    pub exclude_paths: Vec<String>,
}

fn error(code: &str, message: impl Into<String>) -> SearchInputError {
    SearchInputError {
        code: code.to_string(),
        message: message.into(),
    }
}

/// `ts` and `.ts` both become `ts`. Rejects blank and multi-dot tokens.
pub fn extension_token(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    let token = trimmed.strip_prefix('.').unwrap_or(trimmed);
    if token.is_empty()
        || token.contains('.')
        || token.contains('/')
        || token.contains('\\')
        || token.chars().any(char::is_whitespace)
    {
        return None;
    }
    Some(token.to_string())
}

pub fn normalize_slashes(raw: &str) -> String {
    raw.replace('\\', "/")
}

pub fn parse_codebase_search(input: &Value) -> Result<ParsedCodebaseSearch, SearchInputError> {
    let query = match input.get("query") {
        None => return Err(error("INVALID_QUERY", "Missing required field: query")),
        Some(Value::String(query)) if query.trim().is_empty() => {
            return Err(error("INVALID_QUERY", "query must not be empty"));
        }
        Some(Value::String(query)) => query.clone(),
        Some(_) => return Err(error("INVALID_QUERY", "query must be a string")),
    };

    let limit = match input.get("limit") {
        None => 10usize,
        Some(value) => {
            let limit = value.as_u64().filter(|limit| (1..=100).contains(limit));
            let Some(limit) = limit else {
                return Err(error(
                    "INVALID_LIMIT",
                    "limit must be an integer from 1 to 100",
                ));
            };
            usize::try_from(limit).map_err(|_| {
                error("INVALID_LIMIT", "limit must be an integer from 1 to 100")
            })?
        }
    };

    let include_content = match input.get("include_content") {
        None => true,
        Some(Value::Bool(value)) => *value,
        Some(_) => {
            return Err(error("INVALID_FILTER", "include_content must be a boolean"));
        }
    };

    let file_extensions = match input.get("file_extensions") {
        None => Vec::new(),
        Some(value) => parse_extensions(value)?,
    };

    let path_filter = match input.get("path_filter") {
        None => None,
        Some(Value::String(value)) => {
            if value.trim().is_empty() {
                return Err(error("INVALID_FILTER", "path_filter must not be empty"));
            }
            Some(normalize_slashes(value))
        }
        Some(_) => return Err(error("INVALID_FILTER", "path_filter must be a string")),
    };

    let exclude_paths = match input.get("exclude_paths") {
        None => Vec::new(),
        Some(value) => parse_exclude_paths(value)?,
    };

    Ok(ParsedCodebaseSearch {
        query,
        limit,
        include_content,
        file_extensions,
        path_filter,
        exclude_paths,
    })
}

fn parse_extensions(value: &Value) -> Result<Vec<String>, SearchInputError> {
    let items = string_list(value, "file_extensions")?;
    let mut extensions = Vec::with_capacity(items.len());
    for item in items {
        let Some(token) = extension_token(&item) else {
            return Err(error(
                "INVALID_FILTER",
                "file_extensions contains an invalid extension",
            ));
        };
        extensions.push(token);
    }
    Ok(extensions)
}

fn parse_exclude_paths(value: &Value) -> Result<Vec<String>, SearchInputError> {
    Ok(string_list(value, "exclude_paths")?
        .into_iter()
        .map(|item| normalize_slashes(&item))
        .collect())
}

fn string_list(value: &Value, field: &str) -> Result<Vec<String>, SearchInputError> {
    let Some(items) = value.as_array() else {
        return Err(error(
            "INVALID_FILTER",
            format!("{field} must be an array of strings"),
        ));
    };
    if items.is_empty() {
        return Err(error("INVALID_FILTER", format!("{field} must not be empty")));
    }
    let mut parsed = Vec::with_capacity(items.len());
    for item in items {
        let Some(text) = item.as_str() else {
            return Err(error(
                "INVALID_FILTER",
                format!("{field} contains a non-string value"),
            ));
        };
        if text.trim().is_empty() {
            return Err(error(
                "INVALID_FILTER",
                format!("{field} contains an empty value"),
            ));
        }
        parsed.push(text.to_string());
    }
    Ok(parsed)
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn defaults_limit_and_normalizes_extensions_and_slashes() {
        let parsed = parse_codebase_search(&json!({
            "query": "auth",
            "file_extensions": [".ts", "go"],
            "path_filter": r"src\auth",
            "exclude_paths": [r"src\gen"]
        }))
        .unwrap();
        assert_eq!(parsed.limit, 10);
        assert!(parsed.include_content);
        assert_eq!(parsed.file_extensions, vec!["ts".to_string(), "go".to_string()]);
        assert_eq!(parsed.path_filter.as_deref(), Some("src/auth"));
        assert_eq!(parsed.exclude_paths, vec!["src/gen".to_string()]);
    }

    #[test]
    fn rejects_blank_filters_and_out_of_range_limits() {
        assert!(parse_codebase_search(&json!({ "query": "auth", "limit": 101 })).is_err());
        assert!(parse_codebase_search(&json!({ "query": "auth", "limit": null })).is_err());
        assert!(parse_codebase_search(&json!({ "query": "auth", "path_filter": "  " })).is_err());
        assert!(parse_codebase_search(&json!({ "query": "auth", "exclude_paths": [""] })).is_err());
        assert!(parse_codebase_search(&json!({ "query": "auth", "file_extensions": ["tar.ts"] })).is_err());
        assert!(parse_codebase_search(&json!({ "query": "auth", "include_content": "yes" })).is_err());
    }
}
