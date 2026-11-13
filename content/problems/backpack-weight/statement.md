A hiker packs for a day trip. Item `i` weighs `weights[i]` and is worth `values[i]` to them; the backpack holds at most `capacity` in total weight. Each item is either packed once or left behind.

Return the largest total worth of items that fit.

### Example 1

```
weights = [3, 4, 2, 5], values = [30, 50, 15, 60], capacity = 7
Output: 80
```

Items 0 and 1 weigh 7 and are worth 80. Items 2 and 3 also weigh 7 but are worth 75.

### Example 2

```
weights = [5, 6], values = [10, 20], capacity = 4
Output: 0
```

### Example 3

```
weights = [1, 1, 1], values = [5, 5, 5], capacity = 2
Output: 10
```

### Constraints

- 1 ≤ weights.length = values.length ≤ 200
- 1 ≤ weights[i] ≤ 10⁴
- 0 ≤ values[i] ≤ 10⁴
- 0 ≤ capacity ≤ 10⁴
