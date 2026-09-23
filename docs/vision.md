# Vision — Locus

Locus is the local hybrid code-search tool for agents.

- **Identity:** MCP package `@sylphx/locus`, bin `locus`, core library `@sylphx/coderag`, site <https://sylphxai.github.io/locus/>.
- **User:** a coding agent that must find the implementation site of a behavior and the code related to it.
- **Job:** return complete AST-bounded chunks — functions, classes, methods — with path, line range, symbol, score explanation and freshness.
- **Promise:** the default path needs no API key, no Docker and no vector database; optional embeddings are explicit; results are deduplicated and token-budgeted.
- **Defaults:** `fast` uses local AST chunks and lexical ranking; `quality` enables optional vectors and reranking when configured.
- **Boundaries:** Locus owns code-chunk retrieval. Spine owns architecture claims; neither edits files or owns web research.
