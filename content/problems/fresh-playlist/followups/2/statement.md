The station's log is the same: `plays` lists every song played, by song number, and a
**fresh stretch** is a run of consecutive plays in which no song is played twice.

For its podcast the station cuts parts of the archive into episodes. Each request `[a, b]`
takes plays `a` to `b` (both included) and splits them into consecutive **blocks**, each of
which must be a fresh stretch. Return, for each request, the fewest blocks that are
enough, in the order asked.

### Example 1

```
plays = [4, 7, 4, 2, 7, 9, 2], requests = [[0, 6], [1, 4], [3, 5]]
Output: [3, 2, 1]
```

Plays 0 to 6 split as `4, 7 | 4, 2, 7, 9 | 2`. Plays 1 to 4 split as `7, 4, 2 | 7`. Plays
3 to 5 (`2, 7, 9`) are already fresh.

### Example 2

```
plays = [5, 5, 5, 5], requests = [[0, 3], [2, 3]]
Output: [4, 2]
```

### Constraints

- 1 ≤ plays.length ≤ 10⁵
- 1 ≤ plays[i] ≤ 10⁹
- 1 ≤ requests.length ≤ 10⁵
- `requests[j] = [a, b]` with 0 ≤ a ≤ b < plays.length
