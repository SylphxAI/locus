# IDE Integration

Every client uses the same Locus MCP server and `codebase_search` tool. Give
each server entry one explicit repository root so the search boundary is
repeatable.

## Claude Desktop

Add a server entry to `claude_desktop_config.json`:

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

Restart Claude Desktop, then ask it to search for a concrete identifier such as
`authenticate`.

## Claude Code

```bash
claude mcp add locus -- npx -y @sylphx/locus --root=/absolute/path/to/project
```

## Cursor

Create `.cursor/mcp.json` in the workspace:

```json
{
  "mcpServers": {
    "locus": {
      "command": "npx",
      "args": ["-y", "@sylphx/locus", "--root=${workspaceFolder}"]
    }
  }
}
```

Reload Cursor and ask for a `codebase_search` lookup. The result should include
the repository-relative path and line locator rather than a guessed file.

## VS Code clients

For a client that reads `.vscode/mcp.json`, use:

```json
{
  "servers": {
    "locus": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "@sylphx/locus", "--root=${workspaceFolder}"]
    }
  }
}
```

Some extensions use an `mcpServers` key instead; keep the same command and
arguments and follow that extension's schema.

## Multiple repositories

Use one server name and root per repository:

```json
{
  "mcpServers": {
    "locus-frontend": {
      "command": "npx",
      "args": ["-y", "@sylphx/locus", "--root=/projects/frontend"]
    },
    "locus-backend": {
      "command": "npx",
      "args": ["-y", "@sylphx/locus", "--root=/projects/backend"]
    }
  }
}
```

Do not point one process at multiple roots or infer a root from a consumer's
current directory; the root is the repository admission boundary.

## Verify the journey

1. Confirm the client starts the canonical `@sylphx/locus` package.
2. Search for a known symbol or error term.
3. Check that `status` is `ok`, `route` is `rust-tfidf`, and every result
   has `path` plus `startLine`/`endLine`.
4. Prefer `matchedLines` for the exact source lines and retain any `gaps` or
   `warnings` in the consumer record.

An empty `results` array is a valid no-match response. `INVALID_ROOT`,
`INVALID_QUERY`, `INVALID_LIMIT`, `INVALID_SEARCH_OPTIONS`, and
`INDEX_READ_FAILED` are actionable failures; fix the owning input or filesystem
state and retry rather than switching to an unowned search path.

## Troubleshooting

**The server is not listed**

- Validate the client JSON and restart the client.
- Run `npx -y @sylphx/locus --root=/absolute/path/to/project doctor` only when
  using the local launcher; otherwise run `locus doctor` after a global install.
- Confirm the root directory exists and is readable.

**The server starts but search fails**

- Verify the request contains a non-empty query and a limit from `1` through
  `100`.
- Remove filters one at a time to find an invalid empty value.
- Read the returned error code and repair the root, source encoding, or local
  `.coderag` permissions named by the error.
