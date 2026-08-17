# Installation

Locus is a local Rust MCP server. Point one process at one repository and let
the client call `codebase_search`.

## Zero-config client setup

```bash
npx -y @sylphx/locus --root=/absolute/path/to/project
```

The native launcher accepts both `--root=/absolute/path/to/project` and
`--root /absolute/path/to/project`, then supplies that directory as the server's
`CODERAG_ROOT`. A request may also pass `root` explicitly when a client manages
multiple repositories.

The root must exist and be a readable directory. Locus indexes supported source
files into `.coderag/rust-index.json`; it does not require Docker, a database
service, or an embedding credential.

## Global installation

```bash
npm install -g @sylphx/locus
locus --root=/absolute/path/to/project
```

Use the same `codebase_search` request contract after either installation mode.
See the [tools reference](./tools.md) for valid fields, locators, gaps, and
admission errors.

## Environment configuration

`CODERAG_ROOT` is the request-root default when the MCP call omits `root`.
`MCP_TRANSPORT=http` (or `CODERAG_MCP_TRANSPORT=http`) selects the streamable
HTTP transport; otherwise stdio is used. HTTP deployments can additionally set
`MCP_HTTP_HOST`, `MCP_HTTP_PORT`, `MCP_API_KEY`, and `MCP_CORS_ORIGIN`.

Do not put repository-specific credentials in package configuration. Search is
local and deterministic on the Rust route.

## Verify the local journey

1. Start the command with an existing repository root.
2. Ask the MCP client to call `codebase_search` with a concrete identifier such
   as `authenticate` and a small `limit`.
3. Confirm each result has a repository-relative `path` and a
   `startLine`/`endLine` chunk locator; use `matchedLines` when present.
4. If the response has `gaps`, preserve them with the result instead of
   presenting the search as complete. If the engine returns `INVALID_ROOT`,
   `INVALID_QUERY`, `INVALID_LIMIT`, or `INDEX_READ_FAILED`, fix that admission
   cause and retry.

## Troubleshooting

**The server does not start**

- Confirm the selected native package matches the host platform.
- Run `locus doctor` to see which Rust server and sibling `locus-cli` binary
  resolve locally.
- Check that the configured root is an existing directory.

**The search returns no results**

- Use a concrete identifier or error term rather than a blank query.
- Start with no filters, then add one extension/path filter at a time.
- Check `gaps`: `empty_root` and `no_searchable_files` are truthful index
  states, not invitations to switch engines.

**Index refresh fails**

- Repair source-file permissions or invalid source encoding reported by
  `INDEX_READ_FAILED`.
- Repair `.coderag` write permissions or local disk space for
  `INDEX_PERSIST_FAILED`.
- Retry after the owning filesystem problem is fixed; unreadable files are not
  silently converted into empty searchable content.
