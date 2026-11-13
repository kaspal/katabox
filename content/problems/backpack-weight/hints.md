Trying every subset is 2²⁰⁰ choices. Picking items greedily by worth per weight can fail (Example 1). What smaller question could you answer for every capacity?
---
Let `best[c]` be the largest worth that fits in capacity `c` using the items seen so far. How does adding one more item change `best`?
---
For each item (weight w, worth v), for `c` from `capacity` **down** to `w`: `best[c] = max(best[c], best[c − w] + v)`. Going downwards makes sure each item is used at most once.
