## Idea

Use the base problem's union-find with union by size: each root stores how many players its club has. When `a` and `b` have different roots, attach the smaller club to the larger and add the sizes. Clubs never split, so the largest club can only grow, and only the club just formed can become the new largest. Keep a running `biggest`, starting at 1, and update it with the new size after each real merge. No heap or sorted structure is needed.

## Complexity

O((n + m)·α(n)) time with path compression and union by size, where α is the inverse Ackermann function (at most 4 in practice); O(n) extra memory.
