# Tools Reference

Locus exposes one public MCP tool: `codebase_search`. The shipped Rust route
performs deterministic local TF-IDF retrieval over the admitted repository; it
does not silently switch to another indexer or an embedding service.

## `codebase_search`

The repeatable developer journey is:

1. Start Locus with one repository root (for example,
   `npx -y @sylphx/locus --root=/absolute/path/to/project`).
2. Call `codebase_search` with a non-empty query and, when useful, narrow the
   result set with extension/path filters.
3. Open the returned `path` at `matchedLines` or, when no exact content line is
   available, verify the `startLine`–`endLine` chunk range and `symbolName`.
4. Re-run the same query after changing files. The Rust index refreshes before
   the search, so a successful response is tied to the admitted root and its
   current local snapshot.

### Input schema

The tool accepts a JSON object:

| Field | Type | Default | Contract |
| --- | --- | --- | --- |
| `root` | `string` | `CODERAG_ROOT` | Existing readable directory. The request is rejected when it is missing, absent, or a file. |
| `query` | `string` | — | Required after trimming whitespace. Empty queries are rejected. |
| `limit` | `number` | `10` | Integer from `1` through `100`. |
| `include_content` | `boolean` | `true` | Include the indexed chunk text as `snippet`; `false` keeps locators and scores while reducing payload size. |
| `file_extensions` | `string[]` | all indexed extensions | Case-sensitive suffix filters such as `['.ts', '.tsx']`. Values must be non-empty. |
| `path_filter` | `string` | none | Case-sensitive substring filter against the repository-relative path. It must be non-empty when supplied. |
| `exclude_paths` | `string[]` | none | Case-sensitive substring exclusions. Values must be non-empty. |

The Rust index currently admits TypeScript/TSX, JavaScript, Rust, and Markdown
files, while always excluding `node_modules`, `dist`, `target`, `.git`, and
Locus's own `.coderag` metadata.

Example:

```json
{
  "root": "/absolute/path/to/project",
  "query": "authenticate login",
  "limit": 5,
  "include_content": true,
  "file_extensions": [".ts", ".tsx"],
  "path_filter": "src/auth",
  "exclude_paths": ["test"]
}
```

### Result contract

Successful responses are structured JSON. The family envelope adds
`envelope_version`, `tool`, `product`, `product_version`, `route`,
`confidence`, `warnings`, and `gaps` around the Rust payload.

Each result contains:

| Field | Meaning |
| --- | --- |
| `path` | Repository-relative file path. |
| `startLine`, `endLine` | The indexed chunk range. |
| `matchedLines` | Exact source lines containing one or more query tokens, when the match is in file content. |
| `symbolName`, `chunkType` | Best-effort symbol provenance from the chunk extractor. |
| `score`, `matchedTerms`, `scoreComponents` | Deterministic ranking evidence. |
| `snippet` | Chunk text when `include_content` is true. |

```json
{
  "envelope_version": "1",
  "status": "ok",
  "tool": "codebase_search",
  "product": "locus",
  "route": "rust-tfidf",
  "engine": "coderag-core",
  "warnings": [],
  "gaps": [],
  "query": "authenticate login",
  "results": [
    {
      "path": "src/auth/login.ts",
      "startLine": 12,
      "endLine": 29,
      "matchedLines": [12, 18],
      "symbolName": "authenticate",
      "chunkType": "function",
      "matchedTerms": ["authenticate", "login"],
      "score": 1.42,
      "snippet": "..."
    }
  ]
}
```

`status: "ok"` with an empty `results` array means the admitted index found no
matching terms. A non-empty `gaps` array is additional truth, not a result:
`empty_root` means the repository had no non-excluded files, and
`no_searchable_files` means files were present but none were admitted by the
Rust extension/size policy.

### Admission and recovery errors

The engine fails closed at the owning boundary instead of returning a
misleading empty search:

| Code | Meaning | Recovery |
| --- | --- | --- |
| `INVALID_ROOT` | Root is missing, cannot be canonicalized, or is not a directory. | Fix the configured repository path and retry. |
| `INVALID_QUERY` | Query is missing or blank after trimming. | Supply a concrete symbol, identifier, or error term. |
| `INVALID_LIMIT` | Limit is not an integer in `1..=100`. | Correct the request. |
| `INVALID_SEARCH_OPTIONS` | A filter contains an invalid value. | Remove empty filter values and retry. |
| `INDEX_READ_FAILED` / `INDEX_SCAN_FAILED` / `INDEX_METADATA_FAILED` | A source file, directory entry, or file metadata could not be read during refresh. | Repair permissions or the source file, then retry; no partial unreadable content is presented as a successful hit. |
| `INDEX_PERSIST_FAILED` | The local `.coderag` snapshot could not be written. | Fix local storage permissions/space and retry. |
| `HASH_FAILED` / `HASH_PERSIST_FAILED` | The local file-hash manifest could not be read or written. | Fix local storage permissions/space and retry. |

Do not treat `gaps` as proof that a different search engine ran. The response
route remains `rust-tfidf`; unresolved external publish, install, or live state
is outside this tool call.

### Repeatable query patterns

Find an implementation:

```json
{
  "root": "/repo",
  "query": "getUserById",
  "file_extensions": [".ts"],
  "limit": 5
}
```

Find error handling in production code while excluding tests:

```json
{
  "root": "/repo",
  "query": "retry ECONNREFUSED",
  "path_filter": "src",
  "exclude_paths": ["test"],
  "include_content": false
}
```

Start broad, then narrow with the same query. Keep the returned path and line
locator with the consumer's change so another run can verify the same source
claim.

See the [installation guide](./installation.md), [configuration guide](./configuration.md),
and [IDE integration guide](./ide-integration.md) for client setup.
