## Approach: interval DP over the cut positions

Sort the marks and add both ends: `p = [0, sorted marks…, length]`. For a piece from `p[i]` to `p[j]`, the cuts inside it are `p[i+1..j−1]`. Whatever the order, one of them, `p[k]`, is made **first**: it costs the whole piece, `p[j] − p[i]`, and leaves two independent pieces.

```
cost[i][j] = 0                                         if j = i + 1
cost[i][j] = (p[j] − p[i]) + min_{i<k<j} (cost[i][k] + cost[k][j])   otherwise
```

Fill the table by increasing gap `j − i`; the answer is `cost[0][m + 1]`.

### Walkthrough

Example 1, `p = [0, 2, 4, 7, 10]`: `cost[0][2] = 4`, `cost[2][4] = 6`, … and `cost[0][4] = 10 + min(cost[0][1] + cost[1][4], cost[0][2] + cost[2][4], cost[0][3] + cost[3][4]) = 10 + min(0 + 14, 4 + 6, 11 + 0) = 20`.

**Complexity:** O(m³) time (at most about 10⁶ steps), O(m²) space. Trying every order of cuts is factorial; trying every first cut recursively without remembering results is exponential.
