The radio mesh is the same: `n` stations numbered `0` to `n − 1`, and each pair `[a, b]`
in `links` lets stations `a` and `b` talk directly, in both directions, for one **hop**.
This time no link is listed twice.

The network team already knows the fewest number of hops from station `0` to station
`n − 1`. Now they want to know how robust that is: **how many different routes** reach
station `n − 1` from station `0` with exactly that fewest number of hops? Two routes are
different when their sequences of stations differ.

The count can be huge: return it modulo 1 000 003. Return `0` if station `n − 1` cannot be
reached. When `n = 1`, the empty route (zero hops) is the one route.

### Example 1

```
n = 6, links = [[0, 1], [0, 2], [1, 3], [2, 3], [1, 4], [2, 4], [3, 5], [4, 5]]
Output: 4
```

Every fewest route takes 3 hops: 0 → 1 or 2, then → 3 or 4, then → 5. That is 2 · 2 = 4
routes.

### Example 2

```
n = 5, links = [[0, 1], [1, 4], [0, 2], [2, 3], [3, 4]]
Output: 1
```

0 → 1 → 4 takes 2 hops; the route through 2 and 3 takes 3, so it does not count.

### Example 3

```
n = 3, links = [[0, 1]]
Output: 0
```

### Constraints

- 1 ≤ n ≤ 10⁵
- 0 ≤ links.length ≤ 2·10⁵
- `links[i] = [a, b]` with 0 ≤ a, b < n and a ≠ b
- No pair of stations is linked twice (in either order).
