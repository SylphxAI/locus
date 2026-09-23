# Locus

### The exact code chunk for the job

Locus is local code search for coding agents. It indexes the repository on the machine, ranks chunks with BM25, and returns the path, line range, symbol, score, and chunk text.

No API key. No vector database. No Docker. The default transport is stdio.

```bash
npx -y @sylphx/locus --root=/absolute/path/to/project
```

Locus does not search the process working directory. Pass `--root`, set `CODERAG_ROOT`, or pass `root` on the tool call. A tool `root` wins, then the launch `--root`, then `CODERAG_ROOT`.

Claude Code:

```bash
claude mcp add locus -- npx -y @sylphx/locus --root=/absolute/path/to/project
```

Codex:

```bash
codex mcp add locus -- npx -y @sylphx/locus --root=/absolute/path/to/project
```

## Tools

| Tool | Use it when | You get |
| --- | --- | --- |
| `codebase_search` | You can name words that appear in the code | Ranked chunks. Filters apply before the limit. |
| `find_related` | You already have a file and a line | Other chunks that share its tokens |

```json
{
  "query": "authenticate request",
  "limit": 5,
  "file_extensions": ["ts"],
  "path_filter": "src/auth",
  "exclude_paths": ["src/auth/legacy"]
}
```

`login` does not match `authenticate`. Ranking is lexical. Search for the words the code uses.

## What is indexed

`.ts` `.tsx` `.js` `.jsx` `.mjs` `.cjs` `.rs` `.md` `.py` `.go`

Skipped path segments, by exact name: `node_modules`, `dist`, `target`, `.git`. This is not gitignore. Files larger than 1,048,576 bytes are skipped. The index is `.coderag/rust-index.json` and `.coderag/file-hashes.json`.

A line that looks like a symbol starts a chunk. Other files stay one chunk. The patterns are regular expressions, not a syntax tree. [Symbol chunks](https://sylphxai.github.io/locus/guide/ast-chunking).

## Defaults that stay boring

- `limit` defaults to 10 and must be an integer from 1 to 100.
- `include_content` defaults to true. `false` returns locators only.
- There is no `auto`, `fast`, or `quality` search mode.
- Each call refreshes the local index. Unchanged files are a cache hit. That refresh mode is not a search profile you select.
- HTTP exists only when `MCP_TRANSPORT=http`. Agents should use stdio.

## Install surfaces

| Surface | Where |
| --- | --- |
| Claude Code | `claude mcp add locus -- npx -y @sylphx/locus --root=/absolute/path/to/project` |
| Codex | `codex mcp add locus -- npx -y @sylphx/locus --root=/absolute/path/to/project` |
| Cursor | `mcpServers` in `.cursor/mcp.json` or `~/.cursor/mcp.json` |
| VS Code | `servers` in `.vscode/mcp.json` (`${workspaceFolder}` is expanded) |
| Other MCP clients | `mcpServers` in `.mcp.json`, with an absolute `--root` |

Published natives: `@sylphx/locus-darwin-arm64`, `@sylphx/locus-darwin-x64`, `@sylphx/locus-linux-arm64-gnu`, `@sylphx/locus-linux-x64-gnu`. Windows is not a published target.

`npx -y @sylphx/locus doctor` prints the server version and whether `locus-cli` was found.

## Companions

| Product | Job |
| --- | --- |
| [Citra](https://github.com/SylphxAI/citra) | PDF answers with page-level proof |
| [Iris](https://github.com/SylphxAI/iris) | Image facts and pixel evidence |
| [Cue](https://github.com/SylphxAI/cue) | Video timelines and timestamp evidence |
| [Spine](https://github.com/SylphxAI/spine) | Repository architecture and impact |
| [Lookout](https://github.com/SylphxAI/lookout) | Web research with source excerpts |

Locus finds the chunk. Spine maps the architecture. Install only what the task needs.

Docs: <https://sylphxai.github.io/locus/>

## Development

```bash
bun install
bun run build:rust
cargo test
bun run docs:build
```

## License

MIT
