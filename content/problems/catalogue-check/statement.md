A library keeps its catalogue codes in a binary tree so that searching is fast. The tree is a valid **search tree** when, for every node:

- every code in its left subtree is **smaller** than the node's code, and
- every code in its right subtree is **larger** than it.

Equal codes are not allowed. Given `root`, return whether the tree is a valid search tree. An empty tree is valid.

Trees are written in level order, top to bottom and left to right, with `null` for a missing child; the children of a missing node are not listed, and trailing nulls are left out. `[]` is the empty tree.

### Example 1

```
root = [8, 3, 10, 1, 6, null, 14]
Output: true
```

### Example 2

```
root = [8, 3, 10, 1, 9]
Output: false
```

9 is in the left subtree of 8 but is larger than 8, even though it is larger than its parent 3.

### Example 3

```
root = [5, 5]
Output: false
```

### Constraints

- 0 ≤ number of nodes ≤ 10⁴
- -10⁹ ≤ node value ≤ 10⁹
- The tree has at most 1000 levels.
