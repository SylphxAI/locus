---
layout: home

hero:
  name: Locus
  text: The exact code chunk for the job.
  tagline: Local BM25 search for agents. Ranked chunks, on the machine, with no API key and no vector database.
  image:
    src: /logo.svg
    alt: Locus
  actions:
    - theme: brand
      text: Quickstart
      link: /guide/quickstart
    - theme: alt
      text: Tool reference
      link: /mcp/tools

features:
  - title: Two tools
    details: codebase_search ranks chunks with BM25. find_related starts from a file and a line you already trust.
  - title: Filters before the limit
    details: Extension, path, and exclusion filters decide which chunks compete. They do not trim an already-cut list.
  - title: An explicit root
    details: You name the repository. There is no current-directory default and no auto search mode.
  - title: Lexical on purpose
    details: login does not match authenticate. The score is explained term by term. No embeddings on this path.
---

<div class="lk-section">
  <span class="lk-eyebrow">The difference</span>
  <h2 class="lk-h2">A file list is not a location.<br />A ranked chunk is.</h2>
  <p class="lk-lead">Locus indexes the repository on the machine and returns the path, the line range, the symbol, and the chunk text. The ranker is BM25. The tool route id <code>rust-tfidf</code> is the historical name of that path, not a TF-IDF score.</p>
  <div class="lk-compare" style="margin-top:28px">
    <div class="side">
      <h3>What a filename search says</h3>
      <p>A path that contains the word, with no line range, no symbol, and no reason it ranked above the next file.</p>
    </div>
    <div class="side good">
      <h3>What Locus returns</h3>
      <p>path, <span class="lk-cite">startLine</span>, <span class="lk-cite">endLine</span>, symbol, BM25 score, and the chunk text. Filters run before <code>limit</code>.</p>
    </div>
  </div>
</div>

## One search. The words the code uses.

```json
{
  "query": "authenticate request",
  "limit": 5,
  "file_extensions": ["ts"],
  "path_filter": "src/auth",
  "exclude_paths": ["src/auth/legacy"]
}
```

<p class="lk-fine"><code>login</code> does not match <code>authenticate</code>. Pass an absolute <code>--root</code>. Locus will not search the process working directory. The response field <code>route</code> is the string <code>rust-tfidf</code>.</p>

<div class="lk-section">
  <span class="lk-eyebrow">How it works</span>
  <h2 class="lk-h2">Three steps from a question to a line</h2>
  <div class="lk-steps" style="margin-top:26px">
    <div class="lk-step">
      <div class="n">Step 1</div>
      <h3>Point it at the repository</h3>
      <p>One <code>npx</code> line with an absolute <code>--root</code>. A stdio MCP server starts for Claude, Cursor, VS Code, Codex, or any other MCP client. No API key.</p>
    </div>
    <div class="lk-step">
      <div class="n">Step 2</div>
      <h3>Search with tokens you can see</h3>
      <p><code>codebase_search</code> refreshes the local index, scores every chunk that passes the filters, then cuts to <code>limit</code>. The default limit is 10.</p>
    </div>
    <div class="lk-step">
      <div class="n">Step 3</div>
      <h3>Expand from a hit you trust</h3>
      <p><code>find_related</code> takes the repo-relative path and a line from that hit. It ranks other chunks by token overlap. It does not run BM25.</p>
    </div>
  </div>
</div>

<div class="lk-section">
  <span class="lk-eyebrow">What you call</span>
  <h2 class="lk-h2">Two tools. Nothing else runs for you.</h2>
  <p class="lk-lead">There is no search profile named <code>auto</code>, and there is no embedding path hiding behind the lexical one.</p>
  <div class="lk-grid three" style="margin-top:26px">
    <div class="lk-card">
      <div class="lk-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="11" cy="11" r="7" /><path d="m20 20-3.5-3.5" /></svg></div>
      <h3>codebase_search</h3>
      <p>BM25 over symbol and file chunks. <code>k1</code> is 1.2 and <code>b</code> is 0.75. Document frequency comes from the whole index, even when a path filter is set.</p>
    </div>
    <div class="lk-card">
      <div class="lk-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M4 7h16" /><path d="M4 12h10" /><path d="M4 17h7" /><path d="m15 11 4 4-4 4" /></svg></div>
      <h3>find_related</h3>
      <p>Starts from a chunk whose line range contains the line you pass. Other chunks are scored by shared tokens. The route id is <code>rust-related</code>.</p>
    </div>
    <div class="lk-card">
      <div class="lk-icon"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M4 6h16" /><path d="M4 12h16" /><path d="M4 18h10" /></svg></div>
      <h3>Filters, then limit</h3>
      <p><code>file_extensions</code>, <code>path_filter</code>, and <code>exclude_paths</code> choose the candidates. <code>limit</code> is applied after the score sort.</p>
    </div>
  </div>
</div>

<div class="lk-section">
  <span class="lk-eyebrow">Limits</span>
  <h2 class="lk-h2">The defaults are the contract, not a benchmark.</h2>
  <div class="lk-limits" style="margin-top:26px">
    <div class="stat"><div class="num">BM25</div><div class="lbl">the only ranker. No embeddings, no hybrid score, no synonym expansion.</div></div>
    <div class="stat"><div class="num">1 MiB</div><div class="lbl">files larger than 1,048,576 bytes are omitted. <code>.json</code> is not an indexed extension.</div></div>
    <div class="stat"><div class="num">Regex</div><div class="lbl">chunks come from line patterns, not tree-sitter. An indented method stays inside its class chunk.</div></div>
    <div class="stat"><div class="num">.locus</div><div class="lbl">new indexes are <code>.locus/rust-index.json</code>. <code>.coderag/</code> is read only when that file is absent.</div></div>
  </div>
  <p class="lk-fine">Skipped directory segments are <code>node_modules</code>, <code>dist</code>, <code>target</code>, and <code>.git</code>. This is not gitignore. A missing token is an empty result list, not a guessed synonym.</p>
</div>

## Install

```bash
npx -y @sylphx/locus --root=/absolute/path/to/project
```

::: code-group
```json [Claude Desktop / Cursor / VS Code]
{
  "mcpServers": {
    "locus": {
      "command": "npx",
      "args": ["-y", "@sylphx/locus", "--root=/absolute/path/to/project"]
    }
  }
}
```

```bash [Claude Code]
claude mcp add locus -- npx -y @sylphx/locus --root=/absolute/path/to/project
```

```bash [Codex]
codex mcp add locus -- npx -y @sylphx/locus --root=/absolute/path/to/project
```
:::

<div class="lk-cta">
  <h2>Search the tokens. Cite the lines.</h2>
  <p>No API key. <code>codebase_search</code> and <code>find_related</code> are the whole tool surface.</p>
  <p style="margin-top:18px"><a class="VPButton brand" href="./guide/quickstart">Read the quickstart</a> <a class="VPButton alt" href="https://github.com/SylphxAI/locus">Star the repo</a></p>
</div>
