A binary clock shows a number as a row of lights, one per binary digit: a light is on for a 1. The clock maker wants a table of how many lights are on for every number from 0 to `n`.

Return a list whose entry `i` is the number of 1 bits in the binary form of `i`, for `i = 0, 1, …, n`.

### Example 1

```
n = 6
Output: [0, 1, 1, 2, 1, 2, 2]
```

5 is 101 in binary (2 lights), 6 is 110 (2 lights).

### Example 2

```
n = 0
Output: [0]
```

### Example 3

```
n = 9
Output: [0, 1, 1, 2, 1, 2, 2, 3, 1, 2]
```

### Constraints

- 0 ≤ n ≤ 10⁵
