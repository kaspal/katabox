A genealogy club stores small binary family trees as one line of text. A node is written as its value, followed by its left subtree in parentheses and then its right subtree in parentheses, with these rules to keep the text short:

- a node with no children is written as its value alone;
- a missing **right** child is left out, so a node with only a left child is `value(left)`;
- a missing **left** child is written as `()` when there is a right child, so the right child is never mistaken for a left one: `value()(right)`.

Return the text for `root`, or the empty string for an empty tree.

Trees are written in level order, top to bottom and left to right, with `null` for a missing child; the children of a missing node are not listed, and trailing nulls are left out. `[]` is the empty tree.

### Example 1

```
root = [4, 2, 7, 1]
Output: "4(2(1))(7)"
```

### Example 2

```
root = [4, null, 7, 5]
Output: "4()(7(5))"
```

### Example 3

```
root = [-3]
Output: "-3"
```

### Constraints

- 0 ≤ number of nodes ≤ 10⁴
- -1000 ≤ node value ≤ 1000
- The tree has at most 1000 levels.
