---
"@sylphx/locus": patch
---

Name the ranker BM25 on the public surfaces, prefer `LOCUS_ROOT`, and write new indexes under `.locus/`. `CODERAG_ROOT` and `.coderag/` still work when the new names are absent. A legacy cache hit copies both the index and the file hashes into `.locus/`, so the next refresh does not rebuild the repository. The route id stays `rust-tfidf`.
