The radio mesh is the same: `n` stations numbered `0` to `n − 1`, and each pair `[a, b]`
in `links` lets stations `a` and `b` talk directly in both directions, one **hop** per
link. This time the message starts at station `0` and must reach **every** station.

Return an array `hops` of length `n` where `hops[i]` is the fewest hops from station `0`
to station `i`, or `-1` if the message cannot reach station `i` at all. `hops[0]` is `0`.

### Example 1

```
n = 6, links = [[0, 1], [1, 2], [0, 3], [3, 2], [4, 5]]
Output: [0, 1, 2, 1, -1, -1]
```

Stations 4 and 5 only talk to each other, so the message never reaches them.

### Example 2

```
n = 4, links = [[2, 0], [3, 1], [3, 2]]
Output: [0, 3, 1, 2]
```

### Example 3

```
n = 4, links = [[0, 1], [1, 2], [2, 1]]
Output: [0, 1, 2, -1]
```

### Constraints

- 1 ≤ n ≤ 10⁵
- 0 ≤ links.length ≤ 2·10⁵
- `links[i] = [a, b]` with 0 ≤ a, b < n and a ≠ b; a link may be listed twice
