# Symbol chunks

Locus does not parse a syntax tree. It scans lines with regular expressions. Where nothing matches, the file is one chunk of type `file`, from line 1 through the last line.

## Patterns on every indexed file

These match only at column 0. An indented method is not its own chunk. The patterns also run on Markdown, Rust, and Go, so a line that merely looks like one of them becomes a symbol.

| Pattern | `chunkType` |
| --- | --- |
| `export function Name` or `export async function Name` | `function` |
| `export class Name` | `class` |
| `export const Name =` | `const` |
| `function Name` or `async function Name` | `function` |
| `def Name(` | `function` |
| `class Name` | `class` |

`fn paint()` in a `.ts` file matches none of these. It stays inside the file chunk or the surrounding column-0 symbol.

## Rust and Go

These are added only for that extension. Leading whitespace is allowed.

| Extension | Extra patterns | `chunkType` |
| --- | --- | --- |
| `.rs` | `fn`, `struct`, `enum`, `trait` (optional `pub`, `pub(crate)`, `async` on `fn`) | `function`, `struct`, `enum`, `trait` |
| `.go` | `func Name` and `func (recv) Name` | `function` |

## How a span is cut

Symbols are ordered by start line. A chunk runs from its symbol line through the line before the next symbol, or through the end of the file. Two matches with the same name and the same start line collapse to one.

Lines before the first symbol are copied onto the front of every symbol chunk in that file. They are not a separate chunk. `find_related` on one of those preamble lines fails, because no chunk's line range contains them, unless the file had no symbols and is a single `file` chunk.

The chunk text is what `snippet` returns for search. It can be longer than the symbol alone, and the preamble repeats. That is overlap, not a summary.

## What this is not

Not tree-sitter, not an AST, and not a promise that every method is its own hit. TypeScript class methods that are indented stay inside the class chunk. Use `file_extensions` and `path_filter` when the chunk is wider than you wanted, or search a token that only appears in the method.
