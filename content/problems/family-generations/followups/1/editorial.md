## Idea

Compute every depth with one breadth-first pass from the founder (the numbers do not follow the generations, so build children lists first). The answer for `[a, b]` is `depth[a] + depth[b] − 2·depth[c]`, where `c` is their lowest common ancestor. Find `c` with binary lifting: `up[0][v]` is the parent (the founder points to itself) and `up[k][v] = up[k−1][up[k−1][v]]`. First lift the deeper person by the depth difference, one table jump per set bit. If both are now the same person, that is `c`. Otherwise, for each `k` from largest to smallest, jump both by `2ᵏ` whenever the targets differ: they stay just below the meeting point, so `c` is one parent above either of them.

## Complexity

O(n log n) time and memory to build the table, then O(log n) per pair: O((n + q) log n) time in total.
