pub mod cli_bridge;
pub mod codebase_search;
pub mod http_transport;
pub mod launch_args;
pub mod tool_routes;

use rmcp::{
    handler::server::router::tool::ToolRouter,
    handler::server::wrapper::Parameters,
    model::{Implementation, ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router, ErrorData, ServerHandler,
};
use serde_json::json;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CodebaseSearchRequest {
    #[schemars(description = "Repository root. This value wins over the launch --root, which wins over LOCUS_ROOT, which wins over CODERAG_ROOT.")]
    pub root: Option<String>,
    #[schemars(description = "Search query. Required. Blank text is rejected.")]
    pub query: String,
    #[schemars(description = "Maximum number of results after filters. Integer from 1 to 100. Defaults to 10.")]
    pub limit: Option<u64>,
    #[schemars(description = "Include chunk text in snippet. Defaults to true. False returns locators only.")]
    pub include_content: Option<bool>,
    #[schemars(description = "Final file extensions to keep. `ts` and `.ts` match only `.ts`, not `.tsx`. Case-sensitive. Omit to search every indexed file.")]
    pub file_extensions: Option<Vec<String>>,
    #[schemars(description = "Case-sensitive path substring. Backslashes are treated as slashes.")]
    pub path_filter: Option<String>,
    #[schemars(description = "Case-sensitive path substrings to drop before the limit is applied.")]
    pub exclude_paths: Option<Vec<String>>,
}

pub const SERVER_NAME: &str = "locus";
pub const SERVER_VERSION: &str = "0.6.4";
pub const SERVER_INSTRUCTIONS: &str =
    "Locus MCP server (Rust rmcp transport). Use codebase_search for local BM25 retrieval with score explainability. The route id rust-tfidf is the historical name of that BM25 path.";

#[derive(Clone)]
pub struct CoderagMcp {
    pub tool_router: ToolRouter<Self>,
}

impl CoderagMcp {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router]
impl CoderagMcp {
    #[tool(
        description = "Find code chunks related to a known file and line by local token overlap."
    )]
    pub fn find_related(
        &self,
        Parameters(request): Parameters<FindRelatedRequest>,
    ) -> Result<rmcp::model::CallToolResult, ErrorData> {
        let mut args = json!({
            "path": request.path,
            "line": request.line,
            "limit": request.limit.unwrap_or(10),
        });
        if let Some(root) = request.root {
            args["root"] = json!(root);
        }
        codebase_search::find_related(args)
    }

    #[tool(
        description = "Search the codebase with local BM25. Returns ranked chunks with path, lines, symbol, score, and matched terms. Filters apply before the limit. The route id rust-tfidf is the historical name of this path."
    )]
    pub fn codebase_search(
        &self,
        Parameters(request): Parameters<CodebaseSearchRequest>,
    ) -> Result<rmcp::model::CallToolResult, ErrorData> {
        let mut args = json!({
            "query": request.query,
        });
        if let Some(root) = request.root {
            args["root"] = json!(root);
        }
        if let Some(limit) = request.limit {
            args["limit"] = json!(limit);
        }
        if let Some(include_content) = request.include_content {
            args["include_content"] = json!(include_content);
        }
        if let Some(file_extensions) = request.file_extensions {
            args["file_extensions"] = json!(file_extensions);
        }
        if let Some(path_filter) = request.path_filter {
            args["path_filter"] = json!(path_filter);
        }
        if let Some(exclude_paths) = request.exclude_paths {
            args["exclude_paths"] = json!(exclude_paths);
        }
        codebase_search::codebase_search(args)
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct FindRelatedRequest {
    #[schemars(description = "Repository root. This value wins over the launch --root, which wins over LOCUS_ROOT, which wins over CODERAG_ROOT.")]
    pub root: Option<String>,
    #[schemars(description = "Known source path")]
    pub path: String,
    #[schemars(description = "Known source line")]
    pub line: u64,
    #[schemars(description = "Maximum number of related chunks")]
    pub limit: Option<u64>,
}

#[tool_handler]
impl ServerHandler for CoderagMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions(SERVER_INSTRUCTIONS)
            .with_server_info(
                Implementation::new(SERVER_NAME, SERVER_VERSION)
                    .with_description("Locus — local BM25 code search MCP (Rust rmcp transport)")
                    .with_website_url("https://sylphxai.github.io/locus/"),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::CoderagMcp;
    #[test]
    fn exposes_codebase_search_tool_surface() {
        let tools = CoderagMcp::new().tool_router.list_all();
        let names: Vec<_> = tools.iter().map(|tool| tool.name.to_string()).collect();
        assert!(names.contains(&"codebase_search".to_string()));
        assert!(names.contains(&"find_related".to_string()));
    }

    #[test]
    fn server_info_is_brand_sole_locus() {
        use rmcp::ServerHandler;
        use crate::{SERVER_NAME, SERVER_VERSION};
        let info = CoderagMcp::new().get_info();
        let name = info.server_info.name.to_string();
        let version = info.server_info.version.to_string();
        assert_eq!(name, SERVER_NAME);
        assert_eq!(version, SERVER_VERSION);
        assert_eq!(SERVER_NAME, "locus");
        assert!(!name.contains("coderag"));
    }


    #[test]
    fn codebase_search_request_schema_is_generated_for_rmcp_tool() {
        let tools = CoderagMcp::new().tool_router.list_all();
        let tool = tools
            .iter()
            .find(|tool| tool.name == "codebase_search")
            .expect("codebase_search tool");
        let schema = tool.input_schema.as_ref();
        assert!(schema.get("properties").is_some());
        let properties = schema.get("properties").expect("properties");
        assert!(properties.get("query").is_some());
        assert!(properties.get("include_content").is_some());
        assert!(properties.get("file_extensions").is_some());
        assert!(properties.get("path_filter").is_some());
        assert!(properties.get("exclude_paths").is_some());
        let required = schema
            .get("required")
            .and_then(|value| value.as_array())
            .expect("required");
        let required: Vec<_> = required.iter().filter_map(|value| value.as_str()).collect();
        assert!(required.contains(&"query"));
        assert!(!required.contains(&"file_extensions"));
        assert!(!required.contains(&"include_content"));
    }
}
