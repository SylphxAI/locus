# MCP server

Locus is an MCP server with two tools.

| Tool | Question it answers |
| --- | --- |
| `codebase_search` | Which chunks match these tokens? |
| `find_related` | Which chunks overlap this file and line? |

The process name is `locus`. The npm package is `@sylphx/locus`. The registry name is `io.github.SylphxAI/locus`.

## Transport

Stdio is the default. Leave it there for Claude Code, Codex, Cursor, and VS Code.

HTTP is off unless `MCP_TRANSPORT=http` or `CODERAG_MCP_TRANSPORT=http`. It then listens on `http://127.0.0.1:8080/mcp`, with a health check at `/mcp/health`. See [configuration](/mcp/configuration).

## What the server will not do

- Search the current working directory when you forget the root.
- Expand synonyms, embeddings, or a `quality` profile.
- Watch the filesystem. It refreshes on each call instead.
- Report stale files or unsupported languages inside `warnings` or `gaps`. Those arrays are empty.

Indexing rules are in [how search works](/guide/how-search-works). Arguments are in the [tool reference](/mcp/tools).
