An orchard keeper draws a fruit tree as a binary tree: every node is a branch fork, and its value is the number of apples hanging there. The root is at **level 1**, its children at level 2, and so on.

Return the level whose forks hold the most apples in total. If several levels tie, return the one closest to the root. For an empty tree, return `0`.

Trees are written in level order, top to bottom and left to right, with `null` for a missing child; the children of a missing node are not listed, and trailing nulls are left out. So `[4, 7, 2, null, 1]` is 4 with children 7 and 2, where 7 has only a right child, 1. `[]` is the empty tree.

### Example 1

```
root = [4, 7, 2, null, 1, 8, 3]
Output: 3
```

Level 1 holds 4 apples, level 2 holds 7 + 2 = 9, and level 3 holds 1 + 8 + 3 = 12.

### Example 2

```
root = [6, 3, 3]
Output: 1
```

Levels 1 and 2 both hold 6 apples; level 1 is closer to the root.

### Example 3

```
root = []
Output: 0
```

### Constraints

- 0 ≤ number of nodes ≤ 10⁴
- 0 ≤ node value ≤ 10⁴
- The tree has at most 1000 levels.
