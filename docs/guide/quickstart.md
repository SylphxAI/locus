# Quickstart

## Install

```bash
npx -y @sylphx/locus --root=/absolute/path/to/project
```

For Claude Code:

```bash
claude mcp add locus -- npx -y @sylphx/locus --root=/absolute/path/to/project
```

Then ask one concrete question and inspect the returned locators, route, warnings,
and gaps before relying on the answer.

## Predictable defaults

`fast` uses local AST chunks and TF-IDF. Choose `quality` only when optional
vector search and reranking are configured. External research belongs in a
separate web tool.
