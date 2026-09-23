# Positioning

**One line:** The exact code chunk for the job.

- **User:** a coding agent that needs the implementation site of a behavior, or code related to a file and line.
- **Job:** ranked local chunks with path, line range, symbol, BM25 components, and chunk text.
- **Promise:** on the machine, no API key, no vector database. The root is explicit. The score uses the words in the query, not synonyms.
- **Identity:** `@sylphx/locus`, bin `locus`, site <https://sylphxai.github.io/locus/>. `@sylphx/coderag` is the old library, not this server.
- **Companions:** Citra, Iris, Cue, Spine, and Lookout are separate products. Locus finds the chunk. Spine maps the architecture.

See [vision](./vision.md), [capabilities](./capabilities.md), [tool surface](./TOOL_SURFACE.md), and [evidence](./EVIDENCE_CONTRACT.md).
