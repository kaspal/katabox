A treasure map is a binary tree of caves; each cave holds `val` coins, or a debt when `val` is negative. A **trail** visits a sequence of caves linked parent–child, never visiting a cave twice, and may start and end at any caves (it need not include the root). A trail has at least one cave.

Return the largest total of coins along any trail.

Trees are written in level order, top to bottom and left to right, with `null` for a missing child; the children of a missing node are not listed, and trailing nulls are left out. `[]` is the empty tree.

### Example 1

```
root = [2, -1, 3]
Output: 5
```

The trail 2 → 3.

### Example 2

```
root = [-5, 4, 8, null, null, -2, 6]
Output: 14
```

The trail −2 → 8 → 6 has 12; 8 → 6 has 14; going up to −5 would only lose coins.

### Example 3

```
root = [-3]
Output: -3
```

### Constraints

- 1 ≤ number of nodes ≤ 10⁴
- -1000 ≤ node value ≤ 1000
- The tree has at most 1000 levels.
