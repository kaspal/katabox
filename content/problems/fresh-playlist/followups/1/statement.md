The station's log is the same: `plays` lists every song played, by song number, and a
**fresh stretch** is a run of consecutive plays in which no song is played twice.

Presenters now ask about parts of the archive. Each request `[a, b]` asks for the length
of the longest fresh stretch that lies entirely within plays `a` to `b` (both included).
Return the answer to each request, in the order asked.

### Example 1

```
plays = [4, 7, 4, 2, 7, 9, 2], requests = [[0, 6], [0, 2], [3, 5], [2, 4]]
Output: [4, 2, 3, 3]
```

The whole log's longest fresh stretch is `4, 2, 7, 9` (plays 2 to 5). Within plays 0 to 2
(`4, 7, 4`) the best is 2; plays 3 to 5 (`2, 7, 9`) are fresh; plays 2 to 4 (`4, 2, 7`) are
fresh too.

### Example 2

```
plays = [1, 1, 1], requests = [[0, 2], [1, 1]]
Output: [1, 1]
```

### Constraints

- 1 ≤ plays.length ≤ 10⁵
- 1 ≤ plays[i] ≤ 10⁹
- 1 ≤ requests.length ≤ 10⁵
- `requests[j] = [a, b]` with 0 ≤ a ≤ b < plays.length
