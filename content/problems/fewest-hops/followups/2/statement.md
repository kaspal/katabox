The radio mesh has `n` stations numbered `0` to `n − 1`, but now every link is
**one-way**: a pair `[a, b]` in `links` lets station `a` send to station `b`, not the
other way round. Passing the message along one link is one **hop**.

Before sending, you may **turn at most one link around**: a turned link `[a, b]` then
works only from `b` to `a`, and no longer from `a` to `b`. Every other link keeps its
direction.

Return the fewest hops to get a message from station `0` to station `n − 1`, using the
best choice of link to turn (or none), or `-1` if no choice makes it possible. For
`n = 1` the answer is `0`: the message starts where it has to go.

### Example 1

```
n = 4, links = [[0, 1], [2, 1], [2, 3]]
Output: 3
```

Turn `[2, 1]` around so it works from 1 to 2; then 0 → 1 → 2 → 3 takes three hops.

### Example 2

```
n = 4, links = [[2, 0], [3, 1], [3, 2]]
Output: -1
```

Turning `[2, 0]` lets the message reach station 2, but no link leaves station 2 towards
station 3.

### Example 3

```
n = 4, links = [[0, 1], [1, 2], [2, 1]]
Output: -1
```

### Constraints

- 1 ≤ n ≤ 10⁵
- 0 ≤ links.length ≤ 2·10⁵
- `links[i] = [a, b]` with 0 ≤ a, b < n and a ≠ b; the same link may be listed more than
  once
