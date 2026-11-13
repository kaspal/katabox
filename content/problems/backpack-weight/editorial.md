## Approach: 0/1 knapsack over capacities

Let `best[c]` = the largest worth fitting in capacity `c` using the items processed so far; start with all zeros.

For each item with weight `w` and worth `v`, update the capacities from high to low:

```
for c = capacity down to w:
    best[c] = max(best[c], best[c − w] + v)
```

`best[c − w]` still describes the items **before** this one (it has not been updated yet in this pass, since we go downwards), so the item is counted at most once. The answer is `best[capacity]`.

### Walkthrough

Example 1: after items 0 (3, 30) and 1 (4, 50), `best[7] = 80`. Item 2 (2, 15) gives `best[7] = max(80, best[5] + 15) = max(80, 65)`. Item 3 (5, 60) gives `max(80, best[2] + 60) = max(80, 75)`. The answer stays 80.

**Complexity:** O(n · capacity) time, O(capacity) space: at most 2·10⁶ steps. Trying every subset is O(2ⁿ).
