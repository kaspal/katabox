A topiary hedge grows as a binary tree of branches. Each branch has a vigor score `val`, negative for a sickly branch. The gardener trims the hedge from the tips inward:

- A branch is looked at only after both of its sub-branches have been dealt with.
- If the total vigor of the branch together with **what remains** below it is negative, the branch is cut off with everything below it.
- A total of zero or more is kept.

Return the hedge after all the cuts. If the root itself is cut, the hedge is empty.

Trees are written in level order, top to bottom and left to right, with `null` for a missing child; the children of a missing node are not listed, and trailing nulls are left out. `[]` is the empty tree.

### Example 1

```
root = [3, -5, 2, 1, -4]
Output: [3, null, 2]
```

The branch -4 is cut. What remains of -5 is -5 + 1 = -4, so it is cut too, with the 1 below it. The root keeps 3 + 2 = 5.

### Example 2

```
root = [-1, 1]
Output: [-1, 1]
```

-1 + 1 = 0 is not negative, so nothing is cut.

### Example 3

```
root = [1, -3, null, 2]
Output: [1]
```

### Example 4

```
root = [-1]
Output: []
```

### Constraints

- 0 ≤ number of nodes ≤ 10⁴
- -10⁴ ≤ node value ≤ 10⁴
- The tree has at most 1000 levels.
