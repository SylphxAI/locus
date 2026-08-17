use serde::{Deserialize, Serialize};

pub const ENGINE_NAME: &str = "coderag-core";
pub const ENGINE_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreComponent {
    pub term: String,
    pub term_frequency: f64,
    pub document_frequency: f64,
    pub idf: f64,
    pub bm25: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchHit {
    pub path: String,
    pub score: f64,
    pub matched_terms: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub score_components: Vec<ScoreComponent>,
    pub start_line: Option<u32>,
    pub end_line: Option<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub matched_lines: Vec<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chunk_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexStats {
    pub files_scanned: usize,
    pub chunks_indexed: usize,
    pub elapsed_ms: u64,
    pub refresh_mode: String,
    pub files_changed: usize,
    pub files_removed: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchStats {
    pub elapsed_ms: u64,
    pub route: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolEnvelope {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<SearchHit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<IndexStats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<SearchStats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub gaps: Vec<String>,
}

impl ToolEnvelope {
    pub fn ok_search(query: &str, results: Vec<SearchHit>, elapsed_ms: u64) -> Self {
        Self::ok_search_with_gaps(query, results, elapsed_ms, Vec::new())
    }

    pub fn ok_search_with_gaps(
        query: &str,
        results: Vec<SearchHit>,
        elapsed_ms: u64,
        gaps: Vec<String>,
    ) -> Self {
        Self {
            status: "ok".into(),
            query: Some(query.into()),
            results: Some(results),
            index: None,
            search: Some(SearchStats {
                elapsed_ms,
                route: "rust-tfidf".into(),
            }),
            code: None,
            message: None,
            warnings: Vec::new(),
            gaps,
        }
    }

    pub fn ok_index(stats: IndexStats) -> Self {
        Self::ok_index_with_gaps(stats, Vec::new())
    }

    pub fn ok_index_with_gaps(stats: IndexStats, gaps: Vec<String>) -> Self {
        Self {
            status: "ok".into(),
            query: None,
            results: None,
            index: Some(stats),
            search: None,
            code: None,
            message: None,
            warnings: Vec::new(),
            gaps,
        }
    }

    pub fn error(code: &str, message: &str) -> Self {
        Self {
            status: "error".into(),
            query: None,
            results: None,
            index: None,
            search: None,
            code: Some(code.into()),
            message: Some(message.into()),
            warnings: Vec::new(),
            gaps: Vec::new(),
        }
    }
}
