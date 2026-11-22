Adding up every cell of every rectangle can cost 10⁵ cells per question, 10¹⁰ in all. In
the base problem, prefix sums answered each trip in O(1). Can the same idea work in two
dimensions?
---
Let `P[r][c]` be the total of the rectangle from the top-left corner up to (but not
including) row `r` and column `c`. Any rectangle can be built from four such corners.
---
Fill `P` with `P[r+1][c+1] = miles[r][c] + P[r][c+1] + P[r+1][c] − P[r][c]`. A question
is then `P[v2+1][d2+1] − P[v1][d2+1] − P[v2+1][d1] + P[v1][d1]`.
