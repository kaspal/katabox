A recycling belt carries items of three kinds: `0` is glass, `1` is paper and `2` is plastic. Before the items reach the bins, a robot arm must reorder the belt so that all the glass comes first, then all the paper, then all the plastic.

The arm can only **swap two items** on the belt, and the belt has no room for a second copy. Rearrange `items` in place in a single pass over the belt and return it.

### Example 1

```
items = [2, 0, 1, 2, 0]
Output: [0, 0, 1, 2, 2]
```

### Example 2

```
items = [1]
Output: [1]
```

### Example 3

```
items = [2, 2, 0, 1, 1, 0]
Output: [0, 0, 1, 1, 2, 2]
```

### Constraints

- 1 ≤ items.length ≤ 10⁵
- items[i] is 0, 1 or 2

The judge checks only the returned order; the single pass with swaps is the exercise.
