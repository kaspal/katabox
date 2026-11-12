A message must travel across a mesh of `n` radio stations, numbered `0` to `n − 1`. Each pair `[a, b]` in `links` means stations `a` and `b` can talk to each other directly (in both directions). Passing the message along one link is one **hop**.

What is the fewest number of hops to get a message from station `0` to station `n − 1`? Return `-1` if it cannot get there at all.

### Example 1

```
n = 5, links = [[0, 1], [1, 2], [2, 4], [0, 3], [3, 4]]
Output: 2
```

0 → 3 → 4 takes two hops; the route through 1 and 2 takes three.

### Example 2

```
n = 4, links = [[0, 1], [2, 3]]
Output: -1
```

### Example 3

```
n = 1, links = []
Output: 0
```

The message starts where it has to go.

### Constraints

- 1 ≤ n ≤ 10⁵
- 0 ≤ links.length ≤ 2·10⁵
- `links[i] = [a, b]` with 0 ≤ a, b < n and a ≠ b; a link may be listed twice
