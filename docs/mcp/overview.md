# MCP Server Overview

## What is the Model Context Protocol?

The Model Context Protocol (MCP) is an open standard for connecting AI assistants to external tools and data sources. It enables AI applications like Claude Desktop, Cursor, and VS Code to access custom functionality through a standardized interface.

MCP uses a client-server architecture where:
- **MCP Clients** are AI applications (Claude Desktop, Cursor, etc.)
- **MCP Servers** provide tools and data to the client
- **Tools** are functions the AI can invoke to accomplish tasks

## What is CodeRAG MCP?

Locus (`@sylphx/locus`) is an MCP server that provides deterministic local codebase search to AI assistants. Its shipped Rust route uses code-aware TF-IDF retrieval and returns verifiable file, symbol, and line locators.

**Key Benefits:**

- **Local-first**: No Docker, database service, or embedding credential is required
- **Deterministic**: The same admitted root and query use the Rust TF-IDF route
- **Offline**: Search does not call a remote provider
- **Verifiable**: Results carry chunk ranges, exact matched lines when available, and symbol provenance
- **Recoverable**: Invalid admission and unreadable-source failures are explicit

## How CodeRAG MCP Works

CodeRAG MCP runs as a background process that your AI assistant communicates with via standard input/output. When you ask the AI to search your codebase, it calls the MCP server, which performs the search and returns relevant code snippets.

**Architecture:**

```
┌─────────────────┐
│  AI Assistant   │  (Claude Desktop, Cursor, etc.)
│  (MCP Client)   │
└────────┬────────┘
         │ MCP Protocol (stdio)
         │
┌────────▼────────┐
│   CodeRAG MCP   │  Provides codebase_search tool
│   MCP Server    │
└────────┬────────┘
         │
┌────────▼────────┐
│   .coderag/     │  Local Rust index snapshot
│ rust-index.json │
└─────────────────┘
```

**Workflow:**

1. **Admission**: The request must name an existing readable repository directory and a non-empty query.
2. **Refresh**: The Rust engine builds or incrementally refreshes `.coderag/rust-index.json`.
3. **Search**: The MCP client calls `codebase_search` with a bounded limit and optional path filters.
4. **Results**: The server returns structured ranked hits with file/symbol/chunk locators and explicit gaps.

## Available Tool: codebase_search

CodeRAG MCP provides a single tool: `codebase_search`

**Search mode:**

- **Keyword search**: deterministic TF-IDF ranking with code-aware tokenization.
  Use specific identifiers, function names, or error terms such as
  `getUserById authentication`.

**Key Features:**

- **Fast Ranking**: Rust TF-IDF with code-aware tokenization
- **Smart Filtering**: Filter by file extension, path pattern, or exclude paths
- **Evidence-Aware**: Returns chunk ranges, exact matched lines, symbol names, and scores
- **Chunking**: Splits supported source at recognized function/class/const boundaries
- **Structured Output**: `status`, `results`, `warnings`, and `gaps` remain machine-readable

## Use Cases with AI Assistants

**Before Implementation:**

```
Human: "Add JWT authentication to the API"
AI: Uses codebase_search("authentication JWT") to find existing auth patterns
AI: Implements new feature following existing patterns
```

**Code Understanding:**

```
Human: "How does error handling work in this project?"
AI: Uses codebase_search("error handling try catch") to find examples
AI: Explains the error handling patterns used
```

**Debugging:**

```
Human: "Why is the database connection failing?"
AI: Uses codebase_search("database connection retry") to find relevant code
AI: Identifies issue and suggests fix
```

**Refactoring:**

```
Human: "Extract common validation logic into a utility"
AI: Uses codebase_search("validation schema") to find all validation code
AI: Creates utility and updates references
```

## Indexed files

The Rust route currently indexes TypeScript/TSX, JavaScript, Rust, and Markdown
files. It excludes `node_modules`, `dist`, `target`, `.git`, and its own
`.coderag` metadata. Unsupported files are not presented as searchable content;
an empty or unsupported-only repository is reported through the response
`gaps` array.

## Next Steps

- [Installation Guide](./installation.md) - Install and run CodeRAG MCP
- [Configuration Guide](./configuration.md) - Configure for your AI assistant
- [Tools Reference](./tools.md) - Detailed tool documentation
- [IDE Integration](./ide-integration.md) - Setup for specific IDEs
