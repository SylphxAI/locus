# Languages

Locus indexes by the final extension. The comparison is case-sensitive. `.TS` is not indexed. `.ts` does not include `.tsx`, `.js` does not include `.jsx`, and `.h` is not indexed at all.

| Extension | Indexed | Symbols |
| --- | --- | --- |
| `.ts` `.tsx` `.js` `.jsx` `.mjs` `.cjs` | Yes | Column-0 patterns only |
| `.py` | Yes | Column-0 patterns, including `def` and `class` |
| `.md` | Yes | The same column-0 patterns. Headings are not symbols |
| `.rs` | Yes | Column-0 patterns, plus indented `fn`, `struct`, `enum`, `trait` |
| `.go` | Yes | Column-0 patterns, plus indented `func`, including methods |

Not indexed: `.json`, `.yaml`, `.yml`, `.toml`, `.html`, `.css`, `.java`, `.c`, `.cpp`, `.h`, `.sh`, `.rb`, `.php`, and everything else.

A filter uses the same final-extension rule. `file_extensions: ["ts"]` keeps `src/app.ts` and drops `src/app.tsx`. `file_extensions: ["TS"]` keeps nothing in a normal tree, because the indexed extension is lowercase `ts`.

`file.tar.ts` is indexed as `ts`. The filter token `tar.ts` is rejected during parsing. Filter with `ts`.

Skip rules are by directory name, not by language: `node_modules`, `dist`, `target`, `.git`.
