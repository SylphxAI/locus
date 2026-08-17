# Configuration

Locus has one search contract and one Rust retrieval route. Configure the
repository boundary and transport; query behavior belongs in the
[`codebase_search` tool contract](./tools.md).

## Repository root

Pass the root on the launcher command:

```bash
npx -y @sylphx/locus --root=/absolute/path/to/project
```

The native launcher accepts `--root=/path` and `--root /path` and exports the
value as `CODERAG_ROOT`. A client may also pass `root` in each tool call. The
engine rejects a missing, nonexistent, unreadable, or non-directory root.

## Stdio (default)

Most desktop clients need no environment variables:

```json
{
  "mcpServers": {
    "locus": {
      "command": "npx",
      "args": ["-y", "@sylphx/locus", "--root=/absolute/path/to/project"]
    }
  }
}
```

If the client owns the repository selection, omit `--root` and provide
`CODERAG_ROOT` or `root` in the request instead.

## Streamable HTTP

Set `MCP_TRANSPORT=http` (or `CODERAG_MCP_TRANSPORT=http`) to serve `/mcp` and
`/mcp/health`:

```bash
MCP_TRANSPORT=http \
CODERAG_ROOT=/absolute/path/to/project \
MCP_HTTP_HOST=127.0.0.1 \
MCP_HTTP_PORT=8080 \
npx -y @sylphx/locus
```

Supported HTTP settings:

| Variable | Default | Meaning |
| --- | --- | --- |
| `MCP_HTTP_HOST` / `CODERAG_MCP_HTTP_HOST` | `127.0.0.1` | Bind address. |
| `MCP_HTTP_PORT` / `CODERAG_MCP_HTTP_PORT` | `8080` | Bind port. |
| `MCP_API_KEY` / `CODERAG_MCP_API_KEY` | unset | Optional key for protected HTTP calls. |
| `MCP_CORS_ORIGIN` / `CODERAG_MCP_CORS_ORIGIN` | unset | Optional explicit CORS origin. |

Keep HTTP on loopback unless the deployment owner has supplied an authenticated
network boundary. Never commit keys or repository credentials.

## Index behavior

The Rust engine persists its local snapshot under `.coderag/rust-index.json` and
refreshes it before a public search. Supported files are TypeScript/TSX,
JavaScript, Rust, and Markdown; `node_modules`, `dist`, `target`, `.git`, and
Locus's own `.coderag` metadata are excluded. Unsupported files are not
silently searched by another engine.

`gaps` in a successful response records an admitted but incomplete search state,
such as `empty_root` or `no_searchable_files`. Read and persistence failures are
returned as explicit error codes; repair the owning filesystem issue and retry.

## Client configuration checklist

1. Use the canonical `@sylphx/locus` package and `locus` server name.
2. Pin one repository root per server entry.
3. Call `codebase_search` with a concrete non-empty query and `limit` in `1..=100`.
4. Preserve `path`, `startLine`/`endLine`, `matchedLines`, `symbolName`, and
   `gaps` when handing results to a consumer.

For client-specific JSON, see [IDE integration](./ide-integration.md).
