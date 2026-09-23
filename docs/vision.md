# Vision

Locus is local code search for an agent that already has a repository and needs the chunk to read.

- **Identity:** `@sylphx/locus`, bin `locus`, MCP `io.github.SylphxAI/locus`, site <https://sylphxai.github.io/locus/>.
- **User:** a coding agent looking for the implementation of a behavior, or for code related to a file and line it already trusts.
- **Job:** return ranked chunks with path, line range, symbol name, BM25 score, and the chunk text.
- **Promise:** the server runs on the machine. No API key, Docker, or vector database. The root is explicit. Filters are exact.
- **Boundary:** Locus does not map architecture (that is Spine), read the web, or edit files. It also does not pretend lexical search understands synonyms.
