Tomás keeps a list of daily step changes and wrote a quick function to add up the days from `a` to `b`. His totals look right most of the time, but now and then they come out a little off, and he cannot see why.

Given an integer array `nums` and two indices `a` and `b`, return the sum of `nums[a]`, `nums[a + 1]`, ..., `nums[b]`. Both ends are included.

The editor starts with that code. Find the bug and fix it.

### Example 1

```
nums = [4, -2, 7, 3, 5], a = 1, b = 3
Output: 8
```

-2 + 7 + 3 = 8.

### Example 2

```
nums = [16, -16, -4, -13, 11], a = 3, b = 4
Output: -2
```

### Example 3

```
nums = [-15, -15, 3, -10, -1], a = 2, b = 4
Output: -8
```

### Constraints

- 1 ≤ nums.length ≤ 10⁵
- −1000 ≤ nums[i] ≤ 1000
- 0 ≤ a ≤ b < nums.length
