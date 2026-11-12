## Approach: union-find over the cells

Give cell `(r, c)` the number `r × w + c`. A **disjoint-set union** keeps, for every cell, a parent pointer; following the pointers leads to the representative of its group.

1. Start with every cell as its own group.
2. For every frozen cell, join it with the cell to its right and the cell below it, when those are frozen too. (Left and up neighbours are covered when the scan reaches them.)
3. Count the frozen cells by representative and sort the counts in decreasing order.

```
for each frozen (r, c):
    if (r, c + 1) is frozen: union(id(r, c), id(r, c + 1))
    if (r + 1, c) is frozen: union(id(r, c), id(r + 1, c))
sizes = count of frozen cells per find(cell)
return sizes sorted descending
```

`find` halves the path as it walks up, and `union` hangs the smaller group under the larger, so each operation costs almost constant time.

### Walkthrough

In Example 1 the joins are (0,0)–(0,1), (0,0)–(1,0), (0,4)–(1,4), (2,2)–(3,2) and (3,1)–(3,2). The groups are {(0,0), (0,1), (1,0)}, {(2,2), (3,1), (3,2)} and {(0,4), (1,4)}: sizes `[3, 3, 2]`.

A flood fill (BFS or DFS from each unvisited frozen cell) gives the same answer in the same time; union-find also handles cells that freeze one at a time.

**Complexity:** O(r · c · α(r · c)) time, practically linear, plus sorting the sizes, and O(r · c) space. Repeatedly letting every cell copy the smallest label of its neighbours until nothing changes is correct, but a long winding pond needs one round per cell along it: O((r · c)²).
