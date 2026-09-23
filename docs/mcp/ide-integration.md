# IDE integration

Every client must end up with an absolute repository root. Locus does not default to the process working directory. `${workspaceFolder}` is safe in VS Code because VS Code expands it. Other clients should get a literal absolute path.

## Claude Code

```bash
claude mcp add locus -- npx -y @sylphx/locus --root=/absolute/path/to/project
```

## Codex

```bash
codex mcp add locus -- npx -y @sylphx/locus --root=/absolute/path/to/project
```

Codex stores MCP servers in `~/.codex/config.toml`. Prefer the command above. Do not hand-write the table unless you have confirmed the keys against your Codex version.

## Cursor

User config `~/.cursor/mcp.json`, or a project file `.cursor/mcp.json`:

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

## VS Code

`.vscode/mcp.json` uses `servers`, not `mcpServers`:

```json
{
  "servers": {
    "locus": {
      "command": "npx",
      "args": ["-y", "@sylphx/locus", "--root=${workspaceFolder}"]
    }
  }
}
```

`${workspaceFolderBasename}` is also expanded by VS Code. It is not a path. Do not pass it as `--root`.

## Portable MCP config

`.mcp.json` at the project root:

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

## Claude Desktop

| OS | File |
| --- | --- |
| macOS | `~/Library/Application Support/Claude/claude_desktop_config.json` |
| Windows | `%APPDATA%\\Claude\\claude_desktop_config.json` |
| Linux | `~/.config/Claude/claude_desktop_config.json` |

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

Restart the app after saving. The Linux path is the one this repository has documented; confirm it if your install uses a different directory.

## HTTP

IDE configs above use stdio. Do not point an editor at HTTP unless you set `MCP_TRANSPORT=http` yourself. The default bind is loopback only.
