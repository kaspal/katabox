A tree house is drawn as a binary tree, root at the top. Standing at a window to the **right** of the tree, you see exactly one node per level: the rightmost node of that level (the others are hidden behind it).

Given `root`, return the values you see, from the top level down.

Trees are written in level order, top to bottom and left to right, with `null` for a missing child; the children of a missing node are not listed, and trailing nulls are left out. `[]` is the empty tree.

### Example 1

```
root = [1, 2, 3, null, 5, null, 4]
Output: [1, 3, 4]
```

### Example 2

```
root = [1, 2, 3, 4]
Output: [1, 3, 4]
```

Level 3 has only the node 4, on the left, but nothing hides it.

### Example 3

```
root = []
Output: []
```

### Constraints

- 0 ≤ number of nodes ≤ 10⁴
- -10⁴ ≤ node value ≤ 10⁴
- The tree has at most 1000 levels.
