---
layout: home
hero:
  name: "Locus"
  text: "The exact code chunk for the job"
  tagline: "Find implementation-ready code with fast local hybrid search and related-code discovery."
  actions:
    - theme: brand
      text: Quickstart
      link: /guide/quickstart
    - theme: alt
      text: Tool reference
      link: /reference/tools
features:
  - title: Local-first
    details: The default path keeps source material on your machine and requires no API key.
  - title: Predictable work
    details: Local AST and lexical search are the default. Vectors and reranking are explicit.
  - title: Citeable output
    details: Results carry source locators, routes, warnings, and gaps so agents can verify claims.
---

## Install

```bash
npx -y @sylphx/locus --root=/absolute/path/to/project
```

Use `locus` from the CLI or connect the same tools to Claude Code, Codex, Cursor,
VS Code, and any MCP client.
