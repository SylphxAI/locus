# Tools

## codebase_search

Search the index with BM25. Filters run before `limit`. The top-level `route` is the string `rust-tfidf`.

### Arguments

| Field | Required | Default | Rule |
| --- | --- | --- | --- |
| `query` | yes | | Non-empty string |
| `root` | no | launch `--root`, then `LOCUS_ROOT`, then `CODERAG_ROOT` | No working-directory default |
| `limit` | no | 10 | Integer from 1 to 100 |
| `include_content` | no | `true` | Boolean |
| `file_extensions` | no | every indexed file | Non-empty array of strings |
| `path_filter` | no | none | Non-empty string |
| `exclude_paths` | no | none | Non-empty array of strings |

Parser failures are MCP invalid params and happen before the index refresh.

| Input | Message |
| --- | --- |
| `query` absent | `Missing required field: query` |
| `query` blank or whitespace | `query must not be empty` |
| `query` not a string | `query must be a string` |
| `limit` missing type, `0`, `101`, or `null` | `limit must be an integer from 1 to 100` |
| `include_content` not a boolean | `include_content must be a boolean` |
| `file_extensions` or `exclude_paths` missing, empty, or not an array of strings | `INVALID_FILTER` naming the field |
| `path_filter` blank or not a string | `INVALID_FILTER` |
| Extension token empty, or containing `.`, `/`, `\`, or whitespace | `file_extensions contains an invalid extension` |

`ts` and `.ts` both mean the final extension `ts`. They do not match `.tsx`. Matching is case-sensitive: `TS` does not match `.ts`. Backslashes in paths become slashes. `path_filter` and `exclude_paths` are case-sensitive substrings of the repo-relative path.

A query that parses but produces no token (for example `...` or `a`) returns status ok and an empty `results` array.

### Result

Wire fields are camelCase.

```json
{
  "envelope_version": "1",
  "status": "ok",
  "tool": "codebase_search",
  "product": "locus",
  "product_version": "0.6.4",
  "engine": "coderag-core",
  "route": "rust-tfidf",
  "warnings": [],
  "gaps": [],
  "confidence": { "kind": "deterministic", "notes": [] },
  "query": "authenticate request",
  "results": [
    {
      "path": "src/auth/session.ts",
      "startLine": 40,
      "endLine": 88,
      "symbolName": "authenticate",
      "chunkType": "function",
      "score": 3.4,
      "matchedTerms": ["authenticate", "request"],
      "scoreComponents": [
        {
          "term": "authenticate",
          "termFrequency": 2,
          "documentFrequency": 4,
          "idf": 1.6,
          "bm25": 2.1
        }
      ],
      "snippet": "export function authenticate(request) { ... }"
    }
  ],
  "search": { "elapsedMs": 4, "route": "rust-tfidf" },
  "index": {
    "filesScanned": 120,
    "chunksIndexed": 640,
    "elapsedMs": 12,
    "refreshMode": "cache_hit",
    "filesChanged": 0,
    "filesRemoved": 0
  }
}
```

The numbers above are the shape, not a recorded trace. `product_version` is whatever server you are running.

`include_content: false` sets `snippet` to `null` and leaves the other hit fields. `warnings` and `gaps` are always empty. `payload`, when present, copies `results`.

`index.refreshMode` is `cache_hit`, `incremental`, or `full`.

### Ranking

Tokens are ASCII letters, digits, and `_`, longer than one character, lowercased. BM25 uses `k1 = 1.2`, `b = 0.75`, and Lucene-style idf:

```text
ln( (N - df + 0.5) / (df + 0.5) + 1 )
```

`N` and `df` are the full index. Filters do not recompute them. Hits sort by score descending. There is no promised tie-break.

## find_related

Find chunks that share tokens with the chunk containing a known line. The top-level `route` is the string `rust-related`.

### Arguments

| Field | Required | Default | Rule |
| --- | --- | --- | --- |
| `path` | yes | | Repo-relative, forward slashes, exactly the indexed `path` |
| `line` | yes | | Positive integer inside a chunk's line range |
| `root` | no | same order as search | Required by the time the call runs |
| `limit` | no | 10 | `0` becomes 1. No 1–100 cap |

Missing `path`, `line`, or root are invalid params before refresh. `line` less than 1 becomes the engine error `line must be a positive integer`.

An empty result is not an ok payload. The MCP error message is `No related chunks found for that location`. That includes a path that does not match, a line that falls in the preamble before the first symbol, and a file that is not indexed.

### Score

Take the seed chunk's unique tokens. Count how many tokens in the candidate occur in that set, including repeated candidate tokens. Divide by the size of the union of the two unique-token sets. A candidate that repeats a shared token can score above 1. This is not Jaccard.

Sort is score descending, then `path` ascending. `snippet` is the first 360 characters of the chunk text. `scoreComponents` is empty. The nested `search.route` stays `rust-tfidf` because both tools share that success envelope. Branch on the top-level `route`.

## Index rules that affect both tools

Indexed final extensions: `ts`, `tsx`, `js`, `jsx`, `mjs`, `cjs`, `rs`, `md`, `py`, `go`.

Skipped path segments: `node_modules`, `dist`, `target`, `.git`. Files larger than 1,048,576 bytes are skipped. Symbol patterns are documented under [symbol chunks](/guide/ast-chunking).
