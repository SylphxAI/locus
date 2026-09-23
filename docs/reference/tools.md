# Tool reference

| Tool | Required | Route |
| --- | --- | --- |
| `codebase_search` | `query` | `rust-tfidf` |
| `find_related` | `path`, `line` | `rust-related` |

`codebase_search` applies `file_extensions`, `path_filter`, and `exclude_paths` before `limit`. Full argument rules, errors, and the JSON shape are in the [MCP tool reference](/mcp/tools).

`warnings` and `gaps` are empty arrays. Do not wait for them to explain a weak result. A weak result is a lexical miss: the query tokens were rare or absent.
