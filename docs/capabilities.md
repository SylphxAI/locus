# Capabilities

## Surfaces

| Surface | Identity |
| --- | --- |
| MCP | `io.github.SylphxAI/locus`, stdio by default, `npx -y @sylphx/locus --root=/absolute/path` |
| CLI launcher | `locus` |
| Engine | Rust crate `coderag-core`, engine name `coderag-core` |

`@sylphx/coderag` is the old TypeScript library. It is not this server.

## Owned

| Capability | Tool | What comes back |
| --- | --- | --- |
| Ranked chunk search | `codebase_search` | Path, lines, symbol, BM25 components, chunk text. Route `rust-tfidf` is the historical id of this BM25 path. |
| Related chunks | `find_related` | Token overlap from a known repo-relative path and line. Route `rust-related`. |
| Local index | both tools, before they search | `.locus/rust-index.json` and `.locus/file-hashes.json`. `.coderag/` is read only when the `.locus` index is absent. |

## Not owned

Embeddings, hybrid rank, synonym expansion, file watching, cross-repo search, gitignore, tree-sitter, SQLite, token budgets, and architecture claims. `warnings` and `gaps` are present and empty. Locus does not fill them with staleness or language coverage.
