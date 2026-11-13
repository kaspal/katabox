A city's streets form a grid. A courier starts at the crossing `(0, 0)` and must reach the crossing `(rows, cols)`; every step goes to the next crossing south (adding 1 to the first number) or east (adding 1 to the second). Some crossings are closed for roadworks: `blocked[i] = [r, c]`, and a walk may not pass through them.

Return the number of different walks. The answer can be huge, so return it **modulo 1 000 003** (a prime): the remainder after dividing by 1 000 003.

### Example 1

```
rows = 2, cols = 2, blocked = [[1, 1]]
Output: 2
```

Of the 6 walks, 4 pass through the middle crossing.

### Example 2

```
rows = 3, cols = 1, blocked = []
Output: 4
```

### Example 3

```
rows = 2, cols = 3, blocked = [[0, 1], [1, 0]]
Output: 0
```

Both first steps are closed.

### Example 4

```
rows = 100000, cols = 100000, blocked = []
Output: 306056
```

### Constraints

- 1 ≤ rows, cols ≤ 10⁵
- 0 ≤ blocked.length ≤ 1000
- 0 ≤ r ≤ rows, 0 ≤ c ≤ cols; the blocked crossings are all different and include neither `(0, 0)` nor `(rows, cols)`.
