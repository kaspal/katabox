The base problem's full table would have 5·10⁴ · 5·10⁴ = 2.5·10⁹ cells: too slow and too
big. But you only need to know whether the distance is at most `k`. Which cells can
still matter?
---
Cell `(i, j)` compares the first `i` letters of `typed` with the first `j` of `meant`.
Their lengths differ by `|i − j|`, so that cell is at least `|i − j|`. If the whole words
differ in length by more than `k`, the answer is already `false`.
---
Fill only the cells with `|i − j| ≤ k`, a band of width `2k + 1` around the diagonal,
treating every cell outside it as `k + 1` (and capping values at `k + 1`). Then check
whether the last cell is at most `k`.
