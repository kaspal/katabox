A weather station records the rainfall of each day in millimetres: `rain[d]` is the rainfall of day `d` (days are numbered from 0).

Farmers send `queries`, each `[first, last]`, asking how much rain fell from day `first` to day `last`, both included. Return the answers in the order of the queries.

### Example 1

```
rain = [3, 0, 7, 2, 5], queries = [[0, 2], [1, 3], [4, 4]]
Output: [10, 9, 5]
```

Days 0 to 2 had 3 + 0 + 7 = 10 mm.

### Example 2

```
rain = [0, 0, 0], queries = [[0, 2]]
Output: [0]
```

### Constraints

- 1 ≤ rain.length ≤ 10⁵
- 0 ≤ rain[d] ≤ 10⁴
- 1 ≤ queries.length ≤ 10⁵
- queries[j] = [first, last] with 0 ≤ first ≤ last < rain.length
