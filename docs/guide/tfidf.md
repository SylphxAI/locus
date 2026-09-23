# BM25

The ranker is BM25. The tool route id `rust-tfidf` is the historical name of this path, not a TF-IDF score.

```text
idf(term) = ln( (N - df + 0.5) / (df + 0.5) + 1 )
bm25(term) = idf * (tf * (k1 + 1)) / (tf + k1 * (1 - b + b * docLen / avgLen))
score = sum of bm25(term) over query tokens that occur in the chunk
```

`k1` is 1.2. `b` is 0.75. `N` is the number of chunks in the whole index. `df` is how many chunks contain the term at least once. `tf` is how many times the term occurs in that chunk, including repeats. `avgLen` is the mean token count per chunk. An empty index uses 1 as `avgLen` so the division is defined.

Query tokens are not de-duplicated first. Repeating a word in the query scores it again.

Filters do not change `N` or `df`. A path filter that leaves one file still uses the rarity of the full index.

The score is not a probability and not a percentage. Higher is a closer lexical match. Two hits can tie. Locus does not document a tie-break.

Tokens are ASCII letters, digits, and underscore, folded to lower case, and must be longer than one character. `id` is a token. `a` is not. `authenticate` does not match `login`.
