# Capabilities — Locus

## Surfaces

| Surface | Identity |
| --- | --- |
| MCP | `io.github.SylphxAI/locus` over stdio, `npx -y @sylphx/locus --root=/abs/path` |
| CLI | `locus` |
| Core library | `@sylphx/coderag` |

## Owned capabilities

| Capability | Tool | Evidence |
| --- | --- | --- |
| Code-chunk search | `codebase_search` | path, line range, symbol, score components, freshness |
| Related code | `find_related` | related chunks from a known file and line |
| Cross-repo search | `codebase_search` over multiple roots | per-repo prefixes and resolved roots |

## Evidence contract

Every result carries path, line range, chunk/symbol identity, ranking route, freshness, warnings and gaps. See [EVIDENCE_CONTRACT.md](./EVIDENCE_CONTRACT.md).

## Not owned

Architecture claims, filesystem mutation, web evidence, and mandatory cloud embeddings or vector databases.
