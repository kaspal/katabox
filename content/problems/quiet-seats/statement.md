A small cinema shows its seat map as a list of `rows`, all the same length: `#` is a sold seat and `.` is a free one.

A free seat is **quiet** when none of the seats around it is sold: the seats to its left and right, the three in the row in front and the three in the row behind. Seats on the edge of the map simply have fewer neighbours.

Return how many quiet seats there are.

### Example 1

```
rows = ["#...", "....", "..#."]
Output: 3
```

The quiet seats are the last two of the first row and the first seat of the last row. Every other free seat touches a sold one, sideways or diagonally.

### Example 2

```
rows = ["...", "...", "..."]
Output: 9
```

### Example 3

```
rows = ["#.#"]
Output: 0
```

The middle seat sits between two sold seats.

### Constraints

- 1 ≤ rows.length ≤ 400
- 1 ≤ rows[i].length ≤ 400, all rows the same length
- Every character is `#` or `.`
