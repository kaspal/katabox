Tomás keeps a weather log for his allotment and wrote a small script to answer questions like "how much rain fell from day 3 to day 9?". The totals looked plausible at first, but his neighbour, who keeps a paper log, keeps finding that they come up short.

`rain[i]` is the rainfall on day `i` (days are numbered from 0). Each entry of `queries` is a pair `[l, r]`. Return a list with one number per query, in the same order: the total rainfall from day `l` to day `r`, both days included.

The editor starts with that code. Find the bug and fix it.

### Example 1

```
rain = [3, 0, 5, 2, 7], queries = [[0, 2], [1, 3], [4, 4]]
Output: [8, 7, 7]
```

Days 0 to 2 give 3 + 0 + 5 = 8; days 1 to 3 give 0 + 5 + 2 = 7; day 4 alone gives 7.

### Example 2

```
rain = [4,1,7,7], queries = [[3,3],[1,1]]
Output: [7,1]
```

### Example 3

```
rain = [1,5,2,4], queries = [[2,2],[0,1]]
Output: [2,6]
```

### Constraints

- 1 ≤ rain.length ≤ 10⁵
- 0 ≤ rain[i] ≤ 1000
- 1 ≤ queries.length ≤ 10⁴
- Each query is `[l, r]` with 0 ≤ l ≤ r < rain.length
