# Locus (`@sylphx/locus`)

Local Rust TF-IDF search for coding agents. Two tools: `codebase_search` and `find_related`.

```bash
npx -y @sylphx/locus --root=/absolute/path/to/project
```

Canonical package `@sylphx/locus`, bin `locus`, MCP name `io.github.SylphxAI/locus`. Docs: <https://sylphxai.github.io/locus/>.

Do not install `@sylphx/coderag-mcp`. That id is retired.

## Root

Locus does not default to the current directory.

1. `root` on the tool call
2. Launch `--root` (`--root=PATH` or `--root PATH`)
3. `CODERAG_ROOT`

Launch arguments are only `doctor` and `--root`.

## codebase_search

Required `query` (non-empty string). `limit` defaults to 10 and must be an integer from 1 to 100. `include_content` defaults to true.

Optional filters run before the limit:

- `file_extensions`: `ts` and `.ts` match only `.ts`, not `.tsx`. Case-sensitive. Empty arrays are rejected.
- `path_filter`: case-sensitive path substring. Backslashes become slashes.
- `exclude_paths`: same substring rule, applied as exclusions.

Ranking is BM25 (`k1 = 1.2`, `b = 0.75`) over ASCII tokens longer than one character. `login` does not find `authenticate`. The result `route` is the string `rust-tfidf`.

## find_related

Pass the repo-relative path (forward slashes) and a positive line inside a chunk. `limit` defaults to 10 and is not capped at 100. The result `route` is the string `rust-related`. An empty match is an error: `No related chunks found for that location`.

## Index

Indexed extensions: `.ts`, `.tsx`, `.js`, `.jsx`, `.mjs`, `.cjs`, `.rs`, `.md`, `.py`, `.go`.

Skipped path segments: `node_modules`, `dist`, `target`, `.git`. Not gitignore. Files larger than 1,048,576 bytes are skipped. Files live at `.coderag/rust-index.json` and `.coderag/file-hashes.json`.

## Platforms

`@sylphx/locus-darwin-arm64`, `@sylphx/locus-darwin-x64`, `@sylphx/locus-linux-x64-gnu`, `@sylphx/locus-linux-arm64-gnu`. Windows is not published.

Default transport is stdio. HTTP listens on `127.0.0.1:8080` only when `MCP_TRANSPORT=http` (or `CODERAG_MCP_TRANSPORT=http`).

```bash
npx -y @sylphx/locus doctor
```
