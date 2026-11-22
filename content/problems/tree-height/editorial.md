## The bug

The base case is wrong:

```python
    if root is None:
        return 1
```

A missing child is not a level, but the starter counts it as one. Every leaf then reports height 2 instead of 1, and that extra 1 is carried all the way up, so every non-empty tree comes out one too tall and the empty tree gets 1 instead of 0.

For `root = [7]`, the starter computes `1 + max(1, 1) = 2`, but a single node has height 1.

## The fix

```python
    if root is None:
        return 0
```

The TypeScript starter has the same base case (`if (root === null) return 1;`); change it to return 0.

**Complexity:** O(n) time, O(h) extra space for the recursion, where h ≤ 1000 is the height.
