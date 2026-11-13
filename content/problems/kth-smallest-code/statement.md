The library's catalogue codes are stored in a valid search tree: for every node, the codes in its left subtree are smaller and the codes in its right subtree are larger, and all codes are different.

Given `root` and `k`, return the `k`-th smallest code (`k = 1` is the smallest).

Trees are written in level order, top to bottom and left to right, with `null` for a missing child; the children of a missing node are not listed, and trailing nulls are left out. `[]` is the empty tree.

### Example 1

```
root = [8, 3, 10, 1, 6, null, 14], k = 3
Output: 6
```

The codes in order are 1, 3, 6, 8, 10, 14.

### Example 2

```
root = [8, 3, 10, 1, 6, null, 14], k = 6
Output: 14
```

### Example 3

```
root = [5], k = 1
Output: 5
```

### Constraints

- 1 ≤ number of nodes ≤ 10⁴, and the tree is a valid search tree.
- -10⁹ ≤ node value ≤ 10⁹
- 1 ≤ k ≤ number of nodes
- The tree has at most 1000 levels.
