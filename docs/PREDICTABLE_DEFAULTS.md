# Predictable defaults

There is one search path. It is local BM25 over the Rust index. There is no `fast`, `quality`, or `auto` search profile.

- `limit` defaults to 10.
- `include_content` defaults to true.
- The repository root is never implied by the process working directory.
- Dependencies and build trees named `node_modules`, `dist`, `target`, or `.git` are skipped. This is not gitignore.
- Optional HTTP is off unless `MCP_TRANSPORT=http`.

The index refresh mode accepts the internal string `auto` as an alias for "use the cache when the file hashes match". Callers do not pass that string. It is not a search default.
