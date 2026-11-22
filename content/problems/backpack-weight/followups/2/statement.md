The hiker's problem is the same: item `i` weighs `weights[i]` and is worth `values[i]`,
each item is packed once or left behind, and the total weight must be at most
`capacity`. Return the largest total worth of items that fit.

What changes is the scale. The backpack is **huge** (`capacity` up to 10⁹ and weights up
to 10⁷), but every item is worth at most 100.

### Example 1

```
weights = [3000000, 4000000, 2000000, 5000000], values = [30, 50, 15, 60], capacity = 7000000
Output: 80
```

Items 0 and 1 weigh 7 000 000 and are worth 80.

### Example 2

```
weights = [140892, 596854, 888599, 841236], values = [97, 8, 32, 15], capacity = 2078005
Output: 144
```

Items 0, 2 and 3 weigh 1 870 727 and are worth 144; all four would weigh too much.

### Example 3

```
weights = [905036, 993870, 890299, 59299], values = [11, 10, 46, 21], capacity = 2809052
Output: 78
```

### Constraints

- 1 ≤ weights.length = values.length ≤ 100
- 1 ≤ weights[i] ≤ 10⁷
- 0 ≤ values[i] ≤ 100
- 0 ≤ capacity ≤ 10⁹
