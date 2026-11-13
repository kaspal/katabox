A botanical garden's paths form a binary tree: each node is a fountain, and each parent–child link is one path. A walk may start and end at any two fountains, and never uses a path twice; it does not have to pass through the root.

Return the number of paths in the longest possible walk. A garden with zero or one fountain has no walk: return `0`.

Trees are written in level order, top to bottom and left to right, with `null` for a missing child; the children of a missing node are not listed, and trailing nulls are left out. `[]` is the empty tree.

### Example 1

```
root = [6, 4, 7, 2, 5, null, 8]
Output: 4
```

For example 2 → 4 → 6 → 7 → 8.

### Example 2

```
root = [1, 2, null, 3, 4, 5, null, null, 6]
Output: 4
```

The walk 5 → 3 → 2 → 4 → 6 does not touch the root.

### Example 3

```
root = [9]
Output: 0
```

### Constraints

- 0 ≤ number of nodes ≤ 10⁴
- -10⁴ ≤ node value ≤ 10⁴
- The tree has at most 1000 levels.
