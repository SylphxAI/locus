# Positioning — Locus

**One-liner:** The exact code chunk for the job.

- **User:** a coding agent that must find the implementation site of a behavior and the code related to it.
- **Job:** return AST-bounded code chunks with path, line range, symbol, score explanation and freshness.
- **Promise:** the default path is local, needs no API key, no Docker and no vector database, and returns deduplicated, token-budgeted results.
- **Identity:** MCP package `@sylphx/locus`, bin `locus`, core library `@sylphx/coderag`, site <https://sylphxai.github.io/locus/>.
- **Companion tools:** Citra, Iris, Cue, Spine, Locus and Lookout are independent products composed through public MCP and SDK contracts. Locus finds the code chunk; Spine maps the repository architecture.

See [vision.md](./vision.md) and [capabilities.md](./capabilities.md) for the destination and the owned capabilities. [TOOL_SURFACE.md](./TOOL_SURFACE.md) is the tool policy and [EVIDENCE_CONTRACT.md](./EVIDENCE_CONTRACT.md) is the result contract.
