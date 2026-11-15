## Idea

A place is a rank computed within a group, which is one window function:

1. **Partition** the teams by division, so each division is ranked on its own.
2. **Rank** by points, highest first, with ties sharing a place and no gaps after them.
3. **Order** the result for printing.

## The query

```sql
SELECT division, name, points,
       DENSE_RANK() OVER (PARTITION BY division ORDER BY points DESC) AS place
FROM teams
ORDER BY division, place, name, id;
```

## Details that matter

- **`RANK` leaves gaps.** `RANK()` gives a row 1 plus the number of rows strictly ahead of it, so after two teams tied in place 1 the next team is in place 3. In Example 1 United would be 3 and Town 4. The league counts distinct point totals instead, which is `DENSE_RANK()`: one plus the number of distinct higher totals.
- **`ROW_NUMBER` breaks ties.** It numbers 1, 2, 3, ... even for equal points, so teams that should share a place would not.
- **Highest first.** `ORDER BY points DESC` inside `OVER` puts the most points in place 1; without `DESC` the bottom team would win.
- **A full order.** Two teams of a division can have the same points and the same name, so `id` breaks the last tie. `ORDER BY` can use the alias `place`.
- **Without window functions.** A correlated subquery gives the same number: `1 + (SELECT COUNT(DISTINCT u.points) FROM teams u WHERE u.division = t.division AND u.points > t.points)`. It rescans the division for every team.

## Cost

SQLite sorts the `n` teams by division and points once, then numbers them in one pass: `O(n log n)`, plus one more sort for the final order. The correlated subquery is about `O(n²)` in the worst case.
