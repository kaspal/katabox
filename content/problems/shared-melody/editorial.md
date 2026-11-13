## Approach: longest common subsequence

`L[i][j]` = the longest shared melody of the prefixes `first[0..i)` and `second[0..j)`, with `L[0][·] = L[·][0] = 0`.

- If the last notes match, the best melody can end with that note: `L[i][j] = L[i−1][j−1] + 1`.
- Otherwise at least one of the two last notes is unused: `L[i][j] = max(L[i−1][j], L[i][j−1])`.

The answer is `L[n][m]`. Each row only needs the previous one, so two arrays of length m + 1 suffice.

**Complexity:** O(n·m) time (at most 2.25·10⁶ cells), O(m) space. Trying both choices recursively without remembering results is exponential.
