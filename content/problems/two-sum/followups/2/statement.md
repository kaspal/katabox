This time no pair has to hit `target` exactly. Pick two numbers at different positions
`i < j` and return the sum `nums[i] + nums[j]` that is **closest** to `target`.

If two sums are equally close (one below `target`, one above), return the **smaller**
one: in `[1, 5, 9]` with `target = 8` the sums 6 and 10 are both 2 away, so the answer
is 6. Return the sum itself, not the positions.

### Example 1

```
nums = [7, -2, 15, 4], target = 10
Output: 11
```

7 + 4 = 11 is 1 away from 10; no pair adds up to exactly 10.

### Example 2

```
nums = [16, -16, -4, -13], target = 23
Output: 12
```

### Example 3

```
nums = [-15, -15, 3, -10], target = -1
Output: -7
```

3 + (−10) = −7 is 6 away from −1; every other sum is further.

### Constraints

- 2 ≤ nums.length ≤ 10⁵
- −10⁹ ≤ nums[i] ≤ 10⁹
- −2·10⁹ ≤ target ≤ 2·10⁹
