## The bug

When the two pointers find a match, the starter counts it and steps each pointer once:

```python
            count += 1
            i += 1
            j -= 1
```

If the value at `i` or at `j` is repeated, the new positions can hold the same two values, which match again and are counted again. The task counts pairs of values, not pairs of positions.

With `prices = [1, 1, 3, 3], budget = 4`: positions 0 and 3 give 1 + 3 (count 1), then positions 1 and 2 give 1 + 3 again (count 2). The starter returns 2, but the only pair of values is 1 + 3, so the answer is 1.

## The fix

After counting a match, skip every copy of both values:

```python
            count += 1
            left, right = prices[i], prices[j]
            while i < j and prices[i] == left:
                i += 1
            while i < j and prices[j] == right:
                j -= 1
```

The TypeScript starter has the same `i++; j--;` after `count++`; replace it with the same two skipping loops.

**Complexity:** O(n) time, O(1) extra space; each pointer only moves inward.
