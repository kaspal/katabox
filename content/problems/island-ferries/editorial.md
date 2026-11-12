## Approach: union-find

Keep, for every island, a pointer `parent[i]`; the island a chain of pointers ends at is its group's **representative**.

- `find(x)`: follow `parent` until reaching a representative. Pointing each visited island at its grandparent along the way (path halving) keeps the chains short.
- `union(a, b)`: find both representatives; if they differ, hang the smaller group under the larger one (union by size) and count one merge.

Start with every island as its own representative. After processing every line, the number of groups is `n − merges`.

### Walkthrough

`n = 6`, `links = [[0, 3], [4, 5], [3, 1]]`:

| line | representatives | merge? | groups |
| --- | --- | --- | --- |
| | | | 6 |
| [0, 3] | 0, 3 | yes | 5 |
| [4, 5] | 4, 5 | yes | 4 |
| [3, 1] | 0, 1 | yes | 3 |

**Complexity:** O((n + m) α(n)) time, where α is the inverse Ackermann function (at most 4 in practice), and O(n) space. A breadth-first search over an adjacency list is also O(n + m). Repeatedly relabelling islands with the smallest label among their neighbours until nothing changes can need one pass per island of a long chain: O(n·m).
