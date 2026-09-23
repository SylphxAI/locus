# Installation

```bash
npx -y @sylphx/locus --root=/absolute/path/to/project
```

That command is the whole install. The platform package is an optional dependency and is selected by `npx` from the host:

- `@sylphx/locus-darwin-arm64`
- `@sylphx/locus-darwin-x64`
- `@sylphx/locus-linux-x64-gnu`
- `@sylphx/locus-linux-arm64-gnu`

## Clients

```bash
claude mcp add locus -- npx -y @sylphx/locus --root=/absolute/path/to/project
codex mcp add locus -- npx -y @sylphx/locus --root=/absolute/path/to/project
```

Cursor, VS Code, and a portable `.mcp.json` are in [IDE integration](/mcp/ide-integration).

## Doctor

```bash
npx -y @sylphx/locus doctor
```

A good result names the server version and a `locus-cli` path. `engine cli: unavailable` means the native package did not resolve. Install on a published platform and run the command again from a directory where `npx` can see the optional dependency.

## Root

Pass an absolute `--root`. Locus does not fall back to the working directory. The tool argument `root` overrides the launch root, which overrides `CODERAG_ROOT`.

Unknown launch arguments fail. The only accepted ones are `doctor` and `--root`.
