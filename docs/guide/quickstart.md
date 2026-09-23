# Quickstart

## 1. Point Locus at the repository

```bash
npx -y @sylphx/locus --root=/absolute/path/to/project
```

Claude Code:

```bash
claude mcp add locus -- npx -y @sylphx/locus --root=/absolute/path/to/project
```

Codex:

```bash
codex mcp add locus -- npx -y @sylphx/locus --root=/absolute/path/to/project
```

Check the binary:

```bash
npx -y @sylphx/locus doctor
```

## 2. Ask with the words the code uses

```json
{
  "query": "authenticate request",
  "limit": 5,
  "file_extensions": ["ts"],
  "exclude_paths": ["src/legacy"]
}
```

Read `results[].path`, `startLine`, `endLine`, `symbolName`, and `snippet`. The top-level `route` is `rust-tfidf`.

`login` will not find `authenticate`. If the first query misses, search a token you can see in the file.

## 3. Expand from a hit you trust

```json
{
  "path": "src/auth/session.ts",
  "line": 42,
  "limit": 5
}
```

`path` is the repo-relative path from the search hit, with forward slashes. `route` is `rust-related`.

## Defaults

Limit 10. Chunk text on. No API key. No search profile named `auto`. The [tool reference](/mcp/tools) is the contract.
