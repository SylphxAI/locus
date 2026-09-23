---
layout: home

hero:
  name: "Locus"
  text: "The exact code chunk for the job"
  tagline: "Local BM25 search for agents. Ranked chunks, on the machine, with no API key."
  actions:
    - theme: brand
      text: Quickstart
      link: /guide/quickstart
    - theme: alt
      text: Tool reference
      link: /mcp/tools
features:
  - title: Two tools
    details: codebase_search ranks chunks with BM25. find_related starts from a file and line you already trust.
  - title: Filters before the limit
    details: Extension, path, and exclusion filters decide which chunks compete. They do not trim an already-cut list.
  - title: An explicit root
    details: You name the repository. There is no current-directory default and no auto search mode.
  - title: Lexical on purpose
    details: login does not match authenticate. The score is explained term by term. No embeddings on this path.
---

## Install

```bash
npx -y @sylphx/locus --root=/absolute/path/to/project
```

The same server speaks MCP over stdio to Claude Code, Codex, Cursor, VS Code, and any other client. Pass an absolute root. Locus will not guess the working directory.
