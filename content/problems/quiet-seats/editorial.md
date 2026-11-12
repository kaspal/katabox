## Approach: check the 3 × 3 block around each free seat

For every free seat `(r, c)`, look at the cells `(r + dr, c + dc)` for `dr, dc ∈ {-1, 0, 1}` that lie inside the map. The seat is quiet when none of them is `#` (the seat itself is free, so including it changes nothing).

```
count = 0
for r, c over the map where rows[r][c] == '.':
    if no '#' among the in-map cells of the 3 × 3 block around (r, c):
        count += 1
```

### Walkthrough

In `["#...", "....", "..#."]`, the seat at row 0, column 2 has neighbours `.` `.` (row 0) and `.` `.` `.` (row 1): quiet. The seat at row 1, column 1 touches the `#` at row 2, column 2 diagonally: not quiet.

**Complexity:** O(rows × columns) time, with at most 8 checks per seat, and O(1) extra space.
