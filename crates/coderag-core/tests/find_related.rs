use coderag_core::handle_tool;
use serde_json::json;

#[test]
fn finds_related_chunks_from_a_known_location() {
    let fixture = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/benchmark-corpus");
    let indexed = handle_tool("coderag_index", json!({ "root": fixture, "mode": "full" }));
    assert_eq!(indexed.status, "ok");
    let related = handle_tool("locus_find_related", json!({
        "root": fixture,
        "path": "src/auth/login.ts",
        "line": 3,
        "limit": 5
    }));
    assert_eq!(related.status, "ok");
    assert!(!related.results.unwrap_or_default().is_empty());
}
