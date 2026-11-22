The hiker now has **several copies** of each item: item `i` weighs `weights[i]`, is worth
`values[i]`, and there are `counts[i]` identical copies of it. Any number of copies from
`0` to `counts[i]` may be packed. The backpack still holds at most `capacity` in total
weight.

Return the largest total worth of the copies that fit.

### Example 1

```
weights = [3, 4], values = [30, 50], counts = [2, 1], capacity = 10
Output: 110
```

Two copies of item 0 and one of item 1 weigh 3 + 3 + 4 = 10 and are worth 110.

### Example 2

```
weights = [5, 1, 3], values = [15, 63, 97], counts = [2, 2, 3], capacity = 9
Output: 320
```

Both copies of item 1 and two copies of item 2 weigh 8 and are worth 126 + 194 = 320.

### Example 3

```
weights = [1, 1, 1], values = [46, 21, 94], counts = [3, 2, 2], capacity = 14
Output: 368
```

All 7 copies fit.

### Constraints

- 1 ≤ weights.length = values.length = counts.length ≤ 50
- 1 ≤ weights[i] ≤ 10⁴
- 0 ≤ values[i] ≤ 10⁴
- 1 ≤ counts[i] ≤ 100
- 0 ≤ capacity ≤ 5000
