Look at the last letters of the two words. If they are equal, do they need any fix? If not, what are the three ways to deal with the last letter?
---
Let `d[i][j]` be the fixes needed to turn the first `i` letters of `typed` into the first `j` letters of `meant`. Plain recursion recomputes the same `(i, j)` pairs an exponential number of times.
---
`d[i][0] = i`, `d[0][j] = j`. If `typed[i-1] == meant[j-1]`, `d[i][j] = d[i-1][j-1]`; otherwise `d[i][j] = 1 + min(d[i-1][j]` (delete), `d[i][j-1]` (insert), `d[i-1][j-1]` (replace)`)`. Fill the table row by row; two rows are enough.
