This time the array may hold **many** pairs that add up to `target`, or none. Return
**how many** pairs of positions `i < j` have `nums[i] + nums[j] == target`.

Equal values at different positions are different pairs: in `[2, 2, 2]` with
`target = 4` there are three pairs.

### Example 1

```
nums = [3, 1, 2, 2, 3], target = 4
Output: 3
```

The pairs are positions (0, 1), (1, 4) and (2, 3).

### Example 2

```
nums = [-3, 0, -3, 3, 3], target = 3
Output: 2
```

### Example 3

```
nums = [-3, -3, 1, -2], target = 1
Output: 0
```

### Constraints

- 1 ≤ nums.length ≤ 10⁵
- −10⁹ ≤ nums[i] ≤ 10⁹
- −2·10⁹ ≤ target ≤ 2·10⁹
- The answer fits in a 64-bit integer (and in a JavaScript number).
