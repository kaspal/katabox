## Approach: children lists and a level-by-level walk

The parent array tells each person who is above them, but to measure the tree from the top we need the opposite direction. Build `children[p]`, the list of everyone whose parent is `p`, in one pass, and remember the founder (the person with `-1`).

Then walk the tree **one generation at a time** (breadth-first search): the current generation starts as `[founder]`; the next generation is all children of the current one. Every round adds one generation, and the walk stops when a generation is empty.

```
children[p] = [i for i with parent[i] == p]
level = [founder]; count = 0
while level:
    count += 1
    level = [c for p in level for c in children[p]]
return count
```

### Walkthrough

`parent = [2, -1, 1]`: children of 1 = [2], children of 2 = [0]. The walk visits [1], then [2], then [0], then nothing: 3 generations.

**Complexity:** O(n) time and space: every person is listed once as a child and visited once. Walking up from each person separately is O(n · depth), up to about n²/2 steps on a single line of descendants.
