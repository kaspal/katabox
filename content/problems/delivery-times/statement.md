Kofi plans routes for a bicycle courier service and wrote a function that tells riders how long the trip from the depot to each drop-off spot takes. The riders soon noticed that on some days they reached a spot faster than the function said was possible.

There are `n` spots numbered 0 to `n - 1`. Each entry of `roads` is `[a, b, minutes]`: a two-way road between spots `a` and `b` that takes `minutes` to ride in either direction. Two spots may be joined by more than one road. Return a list of `n` numbers where entry `i` is the fewest minutes needed to get from spot `start` to spot `i`, or `-1` if spot `i` cannot be reached. Entry `start` is `0`.

The editor starts with that code. Find the bug and fix it.

### Example 1

```
n = 4, roads = [[0, 1, 10], [0, 2, 2], [2, 1, 3]], start = 0
Output: [0, 5, 2, -1]
```

Going 0 → 2 → 1 takes 2 + 3 = 5 minutes, less than the direct road of 10. No road reaches spot 3.

### Example 2

```
n = 4, roads = [[0,1,2],[3,1,8],[3,0,2],[3,0,7],[3,2,1],[3,1,4]], start = 0
Output: [0,2,3,2]
```

### Example 3

```
n = 4, roads = [[0,1,3],[2,1,4],[0,2,3],[3,2,7]], start = 2
Output: [3,4,0,7]
```

### Constraints

- 1 ≤ n ≤ 10⁴
- 0 ≤ roads.length ≤ 3·10⁴
- Each road is `[a, b, minutes]` with 0 ≤ a, b < n, a ≠ b and 1 ≤ minutes ≤ 1000
- 0 ≤ start < n
