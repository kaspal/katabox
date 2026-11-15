## Idea

Walk the network from `Central` one crossing at a time, and stop after three:

1. **Start** at `Central` with 0 crossings.
2. **Follow** every route that leaves a pier already reached, one crossing more, as long as fewer than 3 crossings have been used.
3. **Keep the fewest** crossings per pier, leave `Central` out and order the result.

A recursive common table expression does the walking. Its first part is the starting row; its second part takes each row found so far and follows the routes out of that pier.

## The query

```sql
WITH RECURSIVE reach(pier, hops) AS (
  SELECT 'Central', 0
  UNION
  SELECT r.to_pier, reach.hops + 1
  FROM reach
  JOIN routes r ON r.from_pier = reach.pier
  WHERE reach.hops < 3
)
SELECT pier, MIN(hops) AS hops
FROM reach
WHERE pier <> 'Central'
GROUP BY pier
ORDER BY hops, pier;
```

## Details that matter

- **`< 3`, not `<= 3`.** The condition is about the row being *extended*: a row with 2 crossings may take one more and reach 3, but a row that already has 3 must stop. With `reach.hops <= 3`, rows with 3 crossings are extended too, so a pier that needs four crossings, like the last one of `Central → A → B → C → D`, ends up on the list with `hops = 4`.
- **The limit is what stops loops.** In Example 1, Central → Island → Central → Island … goes on forever without a limit, and `UNION` alone cannot stop it, because each lap has a different `hops` and so is a new row. The `hops < 3` test ends every walk after three crossings.
- **`UNION` keeps it small.** With `UNION ALL`, `reach` holds one row for every walk of up to three crossings, and in a dense network the number of walks grows like the number of routes per pier cubed. `UNION` discards a `(pier, hops)` row that is already there before following its routes again, so `reach` has at most four rows per pier (0 to 3 crossings).
- **Fewest crossings.** A pier can be reached by walks of different lengths (Island in Example 1 is at 1 and at 2), so `GROUP BY pier` with `MIN(hops)` keeps the shortest.
- **`Central` is not a destination.** A route back to `Central` puts it in `reach` again; the final `WHERE pier <> 'Central'` removes it. The starting row is also removed by that test, so a `Central` with no routes out, as in Example 2, gives an empty result.
- **One-way routes.** Only `from_pier → to_pier` is followed; a route into `Central` does not let you leave by it.

## Cost

`reach` has at most four rows per pier, and each row is joined with the routes leaving its pier. With up to 400 routes that is a few thousand rows at most; without an index on `from_pier` each join scans `routes`, still only around a million comparisons. The grouping and the final sort are `O(p log p)` for `p` piers.
