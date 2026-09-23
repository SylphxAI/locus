# Defaults

| Input | Default | Bound |
| --- | --- | --- |
| `codebase_search.limit` | 10 | Integer 1 through 100 |
| `codebase_search.include_content` | `true` | Boolean. `false` sets `snippet` to `null` |
| `find_related.limit` | 10 | `0` is raised to 1. No 1–100 cap |
| Root | none | Tool `root`, then `--root`, then `CODERAG_ROOT` |
| Transport | stdio | HTTP only when `MCP_TRANSPORT=http` or `CODERAG_MCP_TRANSPORT=http` |
| HTTP bind | `127.0.0.1:8080` | Override with `MCP_HTTP_HOST` / `MCP_HTTP_PORT` (or the `CODERAG_MCP_` names) |
| Max indexed file | 1,048,576 bytes | Not a tool argument |
| Skip path segments | `node_modules`, `dist`, `target`, `.git` | Exact segment. Not gitignore |

No API key is required for stdio. There is no `fast` or `quality` profile.
