# Locus

SylphxAI/locus is the local BM25 code search MCP server for coding agents. The route id `rust-tfidf` is the historical name of that ranker.

## Lifecycle

- State: `active`
- Layer: `tooling`

## What ships

- `@sylphx/locus` (`npx -y @sylphx/locus`) is the server. The bin name is `locus`.
- Retrieval and the MCP server are the Rust crates under `crates/`.
- `packages/core` (`@sylphx/coderag`) is the old TypeScript library. It is not the live server, and changes to it are not published from this branch.
- Docs: <https://sylphxai.github.io/locus/>

## Behavior the docs must keep true

- Public tools are `codebase_search` and `find_related`.
- Ranking is BM25 (`k1 = 1.2`, `b = 0.75`) over ASCII tokens longer than one character. Not embeddings, not hybrid search, and not synonym search.
- There is no user-facing `auto`, `fast`, or `quality` search mode.
- Root is explicit: tool `root`, then launch `--root`, then `LOCUS_ROOT`, then `CODERAG_ROOT`. There is no current-directory default.
- `file_extensions`, `path_filter`, and `exclude_paths` run before `limit`.

## Non-goals

This repository does not own IDE products, web research, architecture graphs, or cloud embedding accounts.
