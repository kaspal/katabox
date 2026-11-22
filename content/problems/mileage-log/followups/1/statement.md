This time the accountant looks after the **whole fleet**. `miles[v][d]` is how many
kilometres van `v` drove on day `d` (vans and days count from 0, and every van has an
entry for every day). Each question `[v1, d1, v2, d2]` asks for the total kilometres of
vans `v1` to `v2` over days `d1` to `d2`, all four bounds included: a rectangle of the
grid.

Return the total of each question, in the order asked.

### Example 1

```
miles = [[10, 0, 5], [7, 3, 2]], queries = [[0, 0, 1, 2], [1, 1, 1, 2], [0, 2, 1, 2]]
Output: [27, 5, 7]
```

The last question covers day 2 of both vans: 5 + 2 = 7.

### Example 2

```
miles = [[4, 18, 2, 8], [3, 15, 14, 15]], queries = [[0, 0, 1, 3], [0, 0, 1, 3], [1, 0, 1, 1]]
Output: [79, 79, 18]
```

### Example 3

```
miles = [[1, 2, 2, 11], [5, 9, 8, 19]], queries = [[0, 1, 0, 3], [1, 2, 1, 3], [0, 2, 0, 3]]
Output: [15, 27, 13]
```

### Constraints

- 1 ≤ vans, days and vans × days ≤ 10⁵ (every van has the same number of days)
- 0 ≤ miles[v][d] ≤ 1000
- 1 ≤ queries.length ≤ 10⁵
- 0 ≤ v1 ≤ v2 < vans and 0 ≤ d1 ≤ d2 < days for each question [v1, d1, v2, d2]
