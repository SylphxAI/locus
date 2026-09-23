# Evidence contract

A successful tool result is JSON. Agents should read `results` and the top-level `route`.

Always present:

- `envelope_version`: `"1"`
- `status`: `"ok"` or the call is an MCP error
- `tool`: `codebase_search` or `find_related`
- `product`: `"locus"`
- `product_version`: the running server version
- `engine`: `"coderag-core"`
- `warnings`: `[]`
- `gaps`: `[]`
- `confidence`: `{ "kind": "deterministic", "notes": [] }`

`warnings` and `gaps` are empty. They are not a staleness report and not a language-coverage report. `confidence.kind` means the rank is a pure function of the index and the query, not that a model checked the answer.

Top-level `route` is a string:

| Tool | `route` |
| --- | --- |
| `codebase_search` | `rust-tfidf` |
| `find_related` | `rust-related` |

The nested `search.route` is `rust-tfidf` for both tools. Branch on the top-level `route`.

Each search hit carries `path`, `startLine`, `endLine`, `score`, `matchedTerms`, and usually `symbolName`, `chunkType`, and `snippet`. Search hits also carry `scoreComponents` (`term`, `termFrequency`, `documentFrequency`, `idf`, `bm25`). `include_content: false` sets `snippet` to `null`.

`payload` duplicates `results`. Read `results`.

A successful call also includes `index` stats from the refresh that just ran (`refreshMode` is `cache_hit`, `incremental`, or `full`).
