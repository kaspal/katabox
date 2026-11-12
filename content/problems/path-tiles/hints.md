Look at the last piece of any paving: it is a square, a short slab or a long slab. What is left in front of it?
---
If the last piece is a square, the rest is any paving of a path of length `n − 1`; a short slab leaves `n − 2`, a long one `n − 3`. So the number of pavings of `n` is the sum of the numbers for `n − 1`, `n − 2` and `n − 3`.
---
Fill a table from the start: `ways[0] = 1` (the empty path, paved in one way), and `ways[i] = ways[i−1] + ways[i−2] + ways[i−3]` (missing terms are 0), taking the remainder each time. Recursion without the table recomputes the same lengths again and again.
