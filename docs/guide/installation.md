# Installation

## Agent install

```bash
npx -y @sylphx/locus --root=/absolute/path/to/project
```

`npx` is the launcher. The server is a native binary from the matching optional dependency. Node is not the search engine.

Published packages:

| Host | Package |
| --- | --- |
| macOS Apple silicon | `@sylphx/locus-darwin-arm64` |
| macOS Intel | `@sylphx/locus-darwin-x64` |
| Linux x64 glibc | `@sylphx/locus-linux-x64-gnu` |
| Linux arm64 glibc | `@sylphx/locus-linux-arm64-gnu` |

Windows is not a published target.

## Check the binary

```bash
npx -y @sylphx/locus doctor
```

Doctor prints the server version and the `locus-cli` path. It does not take `--root`. If the engine binary is missing, the line says it is unavailable.

## Arguments

The process accepts only:

- `doctor`
- `--root=/absolute/path` or `--root /absolute/path`

Anything else, including a size flag, is an unknown argument. The 1,048,576-byte file cap is fixed. It is not a command-line option.

## Root order

1. `root` on the tool call
2. `--root` from the launch command
3. `LOCUS_ROOT`, then `CODERAG_ROOT` when `LOCUS_ROOT` is unset

There is no fallback to the current working directory. A tool `root` does not rewrite the environment.

## Claude Code and Codex

```bash
claude mcp add locus -- npx -y @sylphx/locus --root=/absolute/path/to/project
codex mcp add locus -- npx -y @sylphx/locus --root=/absolute/path/to/project
```

Editor files are in [IDE integration](/mcp/ide-integration).
