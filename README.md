# Locus

### The exact code chunk for the job

Locus gives coding agents fast, local code search that returns complete,
implementation-ready chunks instead of grep dumps or whole-file guesses.

```bash
npx -y @sylphx/locus --root=/absolute/path/to/project
```

For Claude Code:

```bash
claude mcp add locus -- npx -y @sylphx/locus --root=/absolute/path/to/project
```

A `root` on the tool call wins. Otherwise Locus uses the launch `--root`, then `CODERAG_ROOT`.

## The fastest useful workflow

Ask a natural-language question:

```json
{
  "query": "where is user authentication enforced?",
  "limit": 5
}
```

Locus returns ranked AST chunks with file paths, line ranges, symbol names,
score explanations, freshness, and explicit gaps.

## Jobs Locus is built for

| Ask your agent | Locus returns |
| --- | --- |
| “Where is this behavior implemented?” | `codebase_search` |
| “What code is related to this function?” | `find_related` |
| “Search across these repositories.” | cross-repo search |
| “Find the implementation, not tests or declarations.” | ranked code chunks |
| “Give me enough context to edit safely.” | compact, deduplicated results |

## Tool surface

| Tool | Purpose |
| --- | --- |
| `codebase_search` | Local Rust TF-IDF search over code chunks |
| `find_related` | Find related chunks from a known file and line |

The public surface stays small. Indexing is cached locally and refreshed when
the repository changes. Optional embeddings are opt-in and never required for
the default path.

## Predictable defaults

Locus does not hide expensive work behind “auto”.

- `fast` is the default local TF-IDF and AST path.
- `quality` enables optional vectors and reranking when configured.
- `research` is not a Locus mode; use a separate web tool for external sources.
- No Docker, vector database, or API key is required.
- Generated files, dependencies, and build output are excluded by default.

## Why agents use it

- AST boundaries return functions, classes, and methods.
- Rust TF-IDF ranks the default results. Optional embeddings can add semantic evidence when configured.
- Results include score components instead of opaque ordering.
- Token budgets and dedup keep context small.
- Local indexes keep code on the machine.

## Companion MCP tools

| Product | Job |
| --- | --- |
| [Citra](https://github.com/SylphxAI/citra) | PDF answers with page-level proof |
| [Iris](https://github.com/SylphxAI/iris) | Image facts and pixel evidence |
| [Cue](https://github.com/SylphxAI/cue) | Video timelines and timestamp evidence |
| [Spine](https://github.com/SylphxAI/spine) | Repository architecture and impact |
| [Lookout](https://github.com/SylphxAI/lookout) | Web research with source excerpts |

Locus finds the code chunk; Spine maps the repository architecture. Each product
is independent, so install only the tools your agent needs.

## Development

```bash
bun install
bun run build
bun test
cargo test
bun run benchmark:public-proof
bun run benchmark:release-gate
```

## License

MIT
