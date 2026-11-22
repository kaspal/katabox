Amara runs the help desk rota and wants to know how many hours the desk is staffed in a week. Her script merges overlapping shifts and adds up the result, and it looks right most weeks, yet sometimes it reports fewer staffed hours than the rota clearly shows.

Each shift `[start, end]` in `shifts` covers the time from `start` to `end`, a length of `end - start`. Shifts may overlap or touch. Return the total length of time covered by at least one shift.

The editor starts with that code. Find the bug and fix it.

### Example 1

```
shifts = [[5, 6], [1, 4], [3, 9], [12, 15]]
Output: 11
```

The shifts cover 1 to 9 and 12 to 15: 8 + 3 = 11.

### Example 2

```
shifts = [[18,20],[8,10],[15,20]]
Output: 7
```

### Example 3

```
shifts = [[2,4],[11,14],[9,14]]
Output: 7
```

### Constraints

- 1 ≤ shifts.length ≤ 10⁴
- shifts[i] = [start, end] with 0 ≤ start < end ≤ 10⁶
