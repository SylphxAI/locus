# Configuration

Locus has no config file. The launch command, the tool arguments, and a short list of environment variables are the whole surface.

## Root

| Source | Precedence |
| --- | --- |
| Tool argument `root` | Highest |
| `--root` / `--root=PATH` | |
| `LOCUS_ROOT` | |
| `CODERAG_ROOT` | Lowest. Used only when `LOCUS_ROOT` is unset |

All four empty is an error: `root is required (pass it on the tool call, launch with --root, or set LOCUS_ROOT)`. The tool argument does not update either environment variable.

Use an absolute path. A relative path is resolved from the server process, not from the editor buffer.

## Environment

| Variable | Effect |
| --- | --- |
| `LOCUS_ROOT` | Repository root when the tool call and `--root` are absent |
| `CODERAG_ROOT` | Same role as `LOCUS_ROOT`, only when `LOCUS_ROOT` is unset |
| `MCP_TRANSPORT` or `CODERAG_MCP_TRANSPORT` | Set to `http` to serve HTTP. Any other value, or unset, is stdio |
| `MCP_HTTP_HOST` or `CODERAG_MCP_HTTP_HOST` | HTTP bind host. Default `127.0.0.1` |
| `MCP_HTTP_PORT` or `CODERAG_MCP_HTTP_PORT` | HTTP port. Default `8080` |
| `MCP_API_KEY` or `CODERAG_MCP_API_KEY` | Require header `X-API-Key` |
| `MCP_CORS_ORIGIN` or `CODERAG_MCP_CORS_ORIGIN` | Allow one browser origin |
| `LOCUS_RUST_CLI` | Override the path to `locus-cli` |

These variables are not read, because the features do not exist on this server: an embedding API key, a search profile, a token budget, a vector weight.

## HTTP

```bash
MCP_TRANSPORT=http npx -y @sylphx/locus --root=/absolute/path/to/project
```

- MCP endpoint: `http://127.0.0.1:8080/mcp`
- Health: `http://127.0.0.1:8080/mcp/health`
- Auth header: `X-API-Key`, only when a key is set
- Binding a non-loopback host without a key prints a warning and still serves

Prefer stdio for local agents. HTTP is the escape hatch.

## Not configurable

Indexed extensions, the skip list (`node_modules`, `dist`, `target`, `.git`), BM25 constants, and the 1,048,576-byte cap are code. Changing them means changing the server, not an environment variable.
