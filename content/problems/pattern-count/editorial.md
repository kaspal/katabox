## Approach: Knuth–Morris–Pratt

**Prefix function.** For the pattern `p`, let `pi[i]` be the length of the longest proper prefix of `p[0..i]` that is also a suffix of it. It is built left to right: with `k = pi[i − 1]`, while `k > 0` and `p[i] ≠ p[k]`, fall back `k = pi[k − 1]`; if `p[i] = p[k]`, `k++`; then `pi[i] = k`.

**Scanning.** Keep `k`, how many pattern characters currently match the end of the text read so far. For each text character `c`: while `k > 0` and `c ≠ p[k]`, `k = pi[k − 1]`; if `c = p[k]`, `k++`. If `k` equals the pattern length, an occurrence ends here: count it and set `k = pi[k − 1]`, so that overlapping occurrences are found too.

`k` grows by at most one per character and every fall back shrinks it, so the total work is linear.

### Walkthrough

`pattern = "tat"` has `pi = [0, 0, 1]`. In "tatatat", the match reaches 3 at positions 2, 4 and 6; each time `k` falls back to `pi[2] = 1` (the final `t` can start the next match).

**Complexity:** O(n + m) time, O(m) space. Comparing at every position is O(n · m), 10¹⁰ steps for a text of 2·10⁵ `a`s and a pattern of 10⁵.
