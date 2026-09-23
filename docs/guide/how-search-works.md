# How search works

`codebase_search` does four things, in order. Invalid arguments stop at step 1. The index is not touched.

1. Parse the arguments.
2. Resolve the repository root.
3. Refresh the local index.
4. Score every chunk that passes the filters, then cut to `limit`.

## Parse

`query` is required. Missing, blank, and non-string queries are errors. `limit` defaults to 10 and must be an integer from 1 to 100. `include_content` defaults to true and must be a boolean.

Empty arrays, blank filter strings, and extension tokens that contain a dot, a slash, or a space are `INVALID_FILTER`. `ts` and `.ts` are the same token. `tar.ts` is rejected. You filter a double extension by its final piece: `ts`.

A query that parses but tokenizes to nothing (punctuation, or only one-character tokens) is not an error. The result list is empty.

## Root

Tool `root`, then launch `--root`, then `CODERAG_ROOT`. The path must exist. Locus canonicalizes it. Relative paths are resolved by the process that launched the server, which is a poor substitute for an absolute path. Pass an absolute path.

## Refresh

Every search and every `find_related` call refreshes before it reads.

| What it finds | `index.refreshMode` |
| --- | --- |
| File hashes match the snapshot | `cache_hit` |
| Some indexed files changed or disappeared | `incremental` |
| No usable snapshot | `full` |

The MCP call asks for refresh. The strings `refresh` and `auto` are internal aliases for that behavior. They are not tool arguments, and they are not search profiles.

Skipped directory segments, exact match: `node_modules`, `dist`, `target`, `.git`. A file named `target.rs` is kept. A file inside a directory named `target` is not. This is not gitignore, and it does not read `.gitignore`.

Files larger than 1,048,576 bytes are omitted. `.json` is not an indexed extension.

Snapshots:

- `.coderag/rust-index.json`
- `.coderag/file-hashes.json`

Chunk paths inside the index are repo-relative with forward slashes. `find_related` must be given that path, not an absolute path.

## Score, then limit

Document frequency is computed on the full index, before filters. A filter changes which hits return. It does not recompute rarity inside the filtered set.

Tokens are ASCII letters, digits, and `_`, lowercased, longer than one character. BM25 uses `k1 = 1.2` and `b = 0.75`. The formula is on the [TF-IDF page](/guide/tfidf). Hits with a zero score are dropped. The rest sort by score descending. Equal scores have no promised order. `limit` is applied last.

`include_content: false` still ranks the same chunks and sets `snippet` to `null`.

## Related chunks

`find_related` does not run BM25. It finds the first chunk whose line range contains `line`, then scores other chunks by token overlap. The details and the "can be greater than 1" score are in the [tool reference](/mcp/tools).
