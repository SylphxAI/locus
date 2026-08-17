pub mod cli_bridge;
pub mod codebase_search;
pub mod http_transport;
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
    #[schemars(
        description = "Existing readable repository directory; defaults to CODERAG_ROOT when omitted"
    )]
    pub root: Option<String>,
    #[schemars(description = "Non-empty search query")]
    pub query: String,
    #[schemars(description = "Maximum number of results, from 1 through 100")]
    pub limit: Option<u64>,
    #[schemars(description = "Include code snippets in results; defaults to true")]
    pub include_content: Option<bool>,
    #[schemars(description = "Only return paths ending with non-empty extension values")]
    pub file_extensions: Option<Vec<String>>,
    #[schemars(description = "Only return paths containing this non-empty substring")]
    pub path_filter: Option<String>,
    #[schemars(description = "Exclude paths containing these non-empty substrings")]
    pub exclude_paths: Option<Vec<String>>,
}

pub const SERVER_NAME: &str = "locus";
pub const SERVER_VERSION: &str = "0.5.3";
pub const SERVER_INSTRUCTIONS: &str =
    "Locus MCP server (Rust rmcp transport). Use codebase_search for deterministic Rust TF-IDF retrieval with score explainability.";

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
        description = "Search the codebase with Rust TF-IDF retrieval. Returns ranked hits with path, score, matched terms, and score components."
    )]
    pub fn codebase_search(
        &self,
        Parameters(request): Parameters<CodebaseSearchRequest>,
    ) -> Result<rmcp::model::CallToolResult, ErrorData> {
        let mut args = json!({
            "query": request.query,
            "limit": request.limit.unwrap_or(10),
        });
        if let Some(root) = request.root {
            args["root"] = json!(root);
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

#[tool_handler]
impl ServerHandler for CoderagMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions(SERVER_INSTRUCTIONS)
            .with_server_info(
                Implementation::new(SERVER_NAME, SERVER_VERSION)
                    .with_description(
                        "Locus — local-first hybrid code search MCP (Rust rmcp transport)",
                    )
                    .with_website_url("https://github.com/SylphxAI/coderag"),
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
    }

    #[test]
    fn server_info_is_brand_sole_locus() {
        use crate::{SERVER_NAME, SERVER_VERSION};
        use rmcp::ServerHandler;
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
    }
}
