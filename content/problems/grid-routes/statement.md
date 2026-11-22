Ingrid programs the delivery robot in a small warehouse and wants to know how many different routes it can take from the loading dock to the packing table. She added a memo to her counting function to make it fast, and since then some floor plans come back with impossible totals.

The warehouse `floor` is a grid of rows of equal length, where `.` is an open cell and `#` is a shelf. The robot starts in the top-left cell and must reach the bottom-right cell, moving one cell right or one cell down at a time and never entering a shelf. Return the number of different routes modulo 1 000 000 007 (10⁹ + 7); if the start or the end cell is a shelf, the answer is 0.

The editor starts with that code. Find the bug and fix it.

### Example 1

```
floor = ["...", ".#.", "..."]
Output: 2
```

The robot goes around the shelf in the middle, either along the top and right edges or along the left and bottom edges.

### Example 2

```
floor = ["...","..#"]
Output: 0
```

The packing table is blocked by a shelf.

### Example 3

```
floor = ["#..","..."]
Output: 0
```

### Constraints

- 1 ≤ floor.length ≤ 100
- 1 ≤ floor[i].length ≤ 100, and every row has the same length.
- Every character is `.` or `#`.
