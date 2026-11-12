A reading room has several lamps on timers. Lamp `i` is on from second `lamps[i][0]` until just before second `lamps[i][1]`: the interval `[start, end)`. Lamps may overlap, touch or be far apart.

For how many seconds in total is **at least one** lamp on?

### Example 1

```
lamps = [[1, 4], [2, 6], [8, 10]]
Output: 7
```

The first two lamps together light `[1, 6)`, 5 seconds; the third adds 2 more.

### Example 2

```
lamps = [[5, 7], [7, 9]]
Output: 4
```

The lamps touch at second 7, so the light never goes out between 5 and 9.

### Example 3

```
lamps = [[0, 1000000000]]
Output: 1000000000
```

### Constraints

- 1 ≤ lamps.length ≤ 10⁵
- `lamps[i] = [start, end]` with 0 ≤ start < end ≤ 10⁹
