## The bug

The first line builds the hall:

```python
hall = [["."] * cols] * rows
```

`["."] * cols` creates one row list, and `* rows` makes a list of `rows` references to that same row. There is only one row in memory, so `hall[r][c] = "X"` changes column `c` in every row at once. With a single row nothing looks wrong, which is why it passes small tests.

With `rows = 2`, `cols = 1` and `bookings = [[0, 0]]`, the code returns `["X", "X"]`, but only row 0 was booked, so the answer is `["X", "."]`.

## The fix

Create a fresh list for each row:

```python
hall = [["."] * cols for _ in range(rows)]
```

The TypeScript version has the same problem because `fill` puts one array object in every slot; build the rows with `Array.from({ length: rows }, () => new Array<string>(cols).fill("."))` instead.

**Complexity:** O(rows · cols + b) time for b bookings, O(rows · cols) space for the plan.
