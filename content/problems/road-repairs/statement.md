A road is divided into `length` sections, numbered from 0. The council plans repairs: each repair `[a, b, crews]` sends `crews` workers to every section from `a` to `b`, both included. Repairs overlap freely.

Return how many workers end up on each section.

### Example 1

```
length = 5, repairs = [[0, 2, 1], [1, 4, 2], [3, 3, 5]]
Output: [1, 3, 3, 7, 2]
```

Section 3 gets 2 workers from the second repair and 5 from the third.

### Example 2

```
length = 3, repairs = []
Output: [0, 0, 0]
```

### Example 3

```
length = 1, repairs = [[0, 0, 4], [0, 0, 4]]
Output: [8]
```

### Constraints

- 1 ≤ length ≤ 10⁵
- 0 ≤ repairs.length ≤ 10⁵
- 0 ≤ a ≤ b < length and 1 ≤ crews ≤ 100 for each repair [a, b, crews]
