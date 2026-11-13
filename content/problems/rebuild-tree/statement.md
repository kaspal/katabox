An archivist recovered two lists from a damaged family-tree file: the members in **preorder** (a member, then their left family branch, then their right one) and in **inorder** (left branch, the member, right branch). Every member has a different number.

Rebuild the binary tree from `preorder` and `inorder` and return it.

Trees are written in level order, top to bottom and left to right, with `null` for a missing child; the children of a missing node are not listed, and trailing nulls are left out. `[]` is the empty tree.

### Example 1

```
preorder = [3, 9, 20, 15, 7], inorder = [9, 3, 15, 20, 7]
Output: [3, 9, 20, null, null, 15, 7]
```

### Example 2

```
preorder = [1, 2, 3], inorder = [3, 2, 1]
Output: [1, 2, null, 3]
```

### Example 3

```
preorder = [], inorder = []
Output: []
```

### Constraints

- 0 ≤ preorder.length = inorder.length ≤ 10⁴
- All numbers are different, between -10⁶ and 10⁶.
- Both lists come from the same binary tree, which has at most 1000 levels.
