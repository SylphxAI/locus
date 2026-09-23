# Competitive positioning

## Job

Local Rust BM25 retrieval of code chunks for agents, over stdio MCP.

## Wedge

One command, an explicit repository root, and a ranked chunk with a path and a score. No account, no vector database, and no daemon to keep warm.

## What the peers are better at

| Peer | Where it wins | Where Locus is the smaller tool |
| --- | --- | --- |
| ripgrep | Literal speed, every file type, gitignore | Locus returns a symbol-sized window and a score, but only for ten extensions, and only for tokens it can spell |
| Hosted code index | Synonyms, cross-repo, huge monorepos | Locus never leaves the machine and never claims meaning it did not tokenize |
| Editor search | Already open, already scoped to the buffer | Locus is a portable MCP tool, not an editor feature |
| Spine | Architecture, impact, ownership | Locus stops at the chunk |

## Non-goals

A cloud index, a second product inside this repository, or a replacement for an architecture map.

## Install

```bash
npx -y @sylphx/locus --root=/absolute/path/to/project
```

The current npm line is `@sylphx/locus`. Check `npm view @sylphx/locus version` rather than a version frozen in this page.
