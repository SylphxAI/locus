# What is Locus?

Locus is a local search server for coding agents. You give it a repository root. It indexes the source on that machine and returns ranked chunks.

It is a small MCP server, not a retrieval framework. The public tools are `codebase_search` and `find_related`.

## When it helps

Use it when the agent knows words that actually appear in the code, or when it already has a file and a line and wants nearby chunks.

A hit looks like this:

| Field | Meaning |
| --- | --- |
| `path` | Repo-relative path, forward slashes |
| `startLine`, `endLine` | Chunk span |
| `symbolName` | Name from a line-start pattern, when one matched |
| `score` | BM25 for search, token overlap for related chunks |
| `snippet` | Chunk text, or `null` when `include_content` is false |

## When it does not

- The query uses different words from the code. `login` does not match `authenticate`.
- The file extension is outside the indexed set.
- You wanted a tree of callers, an owner, or a cross-repo answer. That is a different tool.
- You wanted embeddings. This server does not compute them.

## What "chunk" means here

Locus is not a tree-sitter indexer. It splits a file when a line matches a symbol pattern, and otherwise keeps the file as one chunk. Indented methods in TypeScript, JavaScript, and Python stay inside the enclosing chunk. Rust and Go patterns can see indented declarations. The exact patterns are on the [symbol chunks](/guide/ast-chunking) page.

## Next

[Install](/guide/installation) it, then use the [quickstart](/guide/quickstart). The contract is the [tool reference](/mcp/tools).
