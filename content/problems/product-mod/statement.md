Tomás tracks how a bacteria culture grows: each hour the population is multiplied by a factor, and he wants the final multiplier. His script agrees with his notebook on short runs, but on long runs with big factors the numbers stop matching.

Given `nums`, return the product of all its values modulo 1 000 000 007 (10⁹ + 7).

The editor starts with that code. Find the bug and fix it.

### Example 1

```
nums = [6, 7, 10]
Output: 420
```

### Example 2

```
nums = [5, 2, 8, 8]
Output: 640
```

### Example 3

```
nums = [2, 6, 3, 5]
Output: 180
```

### Constraints

- 1 ≤ nums.length ≤ 10⁵
- 1 ≤ nums[i] ≤ 10⁹
