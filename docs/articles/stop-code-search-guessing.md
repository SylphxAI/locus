# Stop your agent from guessing which chunk matters

An agent asked to fix login often edits a file that only looks related. The model is not the whole failure. It was handed the wrong window of code.

## What the agent was given

ripgrep returns lines. A whole-file dump returns hundreds of lines that happen to share a word. The model then picks a plausible span.

Locus sits in between, on purpose:

- It indexes ten source extensions on the local disk.
- Where a line looks like a symbol, that symbol becomes the chunk. Otherwise the file is one chunk.
- It ranks those chunks with BM25 and returns the path, the line range, and the text.

It does not understand synonyms. A query for `login` misses `authenticateUser` unless those letters appear. It does not watch the filesystem, embed the repository, or search a second repo. If you need a synonym, search the token the code actually uses, or start from `find_related` once you have one good hit.

## Try it

```bash
claude mcp add locus -- npx -y @sylphx/locus --root=/absolute/path/to/project
```

```json
{
  "query": "authenticate request",
  "limit": 5,
  "exclude_paths": ["test"]
}
```

Then check the path and the lines before letting the agent edit. The [search page](/guide/how-search-works) is the behavior. The [tool reference](/mcp/tools) is the contract.

The checked-in TypeScript benchmark is not a measurement of this server. Do not cite it as one.
