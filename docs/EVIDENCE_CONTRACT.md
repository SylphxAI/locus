# Evidence contract — Locus

Family envelope v1. Locators: repository-relative file path, chunk line range,
exact matched lines when available, and symbol/chunk provenance.
Route: family route metadata; Locus MCP `codebase_search` reports `rust-tfidf`.
Gaps: stale index, unsupported language, empty root.
No `evidence_first` tool. Does not own architecture graph claims (Spine) or FS mutation.

## Implemented family wire fields (v1)

Every tool result includes:

- `envelope_version: "1"`
- `status`, `tool`, `product`, `product_version`
- `route` as the route path (or `{ engine, path? }` for family wrappers)
- `warnings` and `gaps` arrays (may be empty)
- domain payload (often also as top-level twin/results/answer for compatibility)

Schema: `SylphxAI/skills` `schemas/instrument-evidence-envelope.schema.json`.
