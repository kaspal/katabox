Lucía draws family trees for a genealogy club and wants to know how many generations each chart spans, so she knows how tall to print it. Her function gives a number for every chart, but the printed charts keep coming out with an empty row.

Given the `root` of a binary tree, return its height: the number of nodes on the longest path from the root down to a leaf. The empty tree has height 0.

Trees are written in level order, top to bottom and left to right, with `null` for a missing child; the children of a missing node are not listed, and trailing nulls are left out. `[]` is the empty tree.

The editor starts with that code. Find the bug and fix it.

### Example 1

```
root = [5, 3, 8, null, 4]
Output: 3
```

The longest path is 5 → 3 → 4.

### Example 2

```
root = [26, 94, 15, 20]
Output: 3
```

### Example 3

```
root = [-8, -57, 88]
Output: 2
```

### Constraints

- 0 ≤ number of nodes ≤ 10⁴
- -100 ≤ node value ≤ 100
- The tree has at most 1000 levels.
