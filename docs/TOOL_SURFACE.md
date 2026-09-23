# Tool surface

Two tools. Anything else is an internal name.

| Surface | Role |
| --- | --- |
| `codebase_search` | BM25 over local chunks. Filters run before the limit. |
| `find_related` | Token overlap from a known repo-relative path and line. |
| `locus` | Process launcher. Arguments are `doctor` and `--root` only. |
| `@sylphx/locus` | The npm package agents install. |

`coderag_search` and `coderag_index` are engine entry points the MCP process calls. They are not extra public tools.

Do not add a near-duplicate tool that only renames one of these. Do not document a `fast`, `quality`, or `auto` search profile. The internal index refresh string `auto` is not a user default.
