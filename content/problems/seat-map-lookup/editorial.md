## Approach 1: binary search in every row

Every row is sorted, so a binary search per row finds the ticket in O(rows · log columns). Rows whose first seat is bigger than the ticket can be skipped, and so can rows whose last seat is smaller.

## Approach 2: a staircase from the top-right corner

Stand on the top-right seat `(r, c) = (0, columns − 1)` and compare:

- `seats[r][c] = ticket`: found;
- `seats[r][c] > ticket`: every seat below it in column `c` is bigger still, so column `c` cannot hold the ticket: `c--`;
- `seats[r][c] < ticket`: every seat left of it in row `r` is smaller still, so row `r` is out: `r++`.

Each step drops a row or a column, so after at most rows + columns steps you either find the ticket or walk off the map (not found).

### Walkthrough

Example 1, ticket 11: start at 9 (< 11, go down), 12 (> 11, go left), 8 (< 11, go down), 11: found.

**Complexity:** O(rows · log columns) or O(rows + columns), O(1) space.
