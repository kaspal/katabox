A village's lanes branch out from the square like a binary tree: each node is a lamp post, and its value is how bright its lantern would be. For the festival, lanterns are lit, but two posts joined directly by a lane (a parent and its child) may not **both** be lit, or the wiring overloads.

Return the largest total brightness that can be lit.

Trees are written in level order, top to bottom and left to right, with `null` for a missing child; the children of a missing node are not listed, and trailing nulls are left out. `[]` is the empty tree.

### Example 1

```
root = [4, 1, 6, 2, 3, null, 5]
Output: 14
```

Light the root (4) and the posts 2, 3 and 5 two levels down.

### Example 2

```
root = [2, 9, 1, null, null, 7, 8]
Output: 24
```

Light the 9, the 7 and the 8.

### Example 3

```
root = []
Output: 0
```

### Constraints

- 0 ≤ number of nodes ≤ 10⁴
- 0 ≤ node value ≤ 10⁴
- The tree has at most 1000 levels.
