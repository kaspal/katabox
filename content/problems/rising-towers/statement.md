A photographer walks along a street, from left to right, past towers of heights `heights[0], heights[1], …`. For a poster, they want to pick some of the towers, keeping their street order, so that every picked tower is **strictly taller** than the one picked before it.

Return the largest number of towers that can be picked.

### Example 1

```
heights = [5, 1, 8, 3, 6, 9, 4]
Output: 4
```

For example 1, 3, 6, 9.

### Example 2

```
heights = [12, 12, 12]
Output: 1
```

Equal heights do not rise.

### Example 3

```
heights = [9, 8, 2]
Output: 1
```

### Constraints

- 1 ≤ heights.length ≤ 5·10⁴
- 1 ≤ heights[i] ≤ 10⁶
