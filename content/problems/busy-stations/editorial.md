## Idea

The counting is easy once every visit to a station is its own row:

1. **Unpivot** the trips: one row `(trip id, station)` for the start and one for the end, with a round trip's two identical rows merged into one.
2. **Group** those rows by station and count them.
3. **Filter** the groups to 3 trips or more, and order them.

## The query

```sql
SELECT station, COUNT(*) AS trips
FROM (
  SELECT id, start_station AS station FROM trips
  UNION
  SELECT id, end_station FROM trips
)
GROUP BY station
HAVING COUNT(*) >= 3
ORDER BY trips DESC, station;
```

## Details that matter

- **`UNION`, not `UNION ALL`.** `UNION ALL` keeps both halves as they are, so a round trip adds 2 to its station. In Example 2 Bridge would then have 4 and be listed, although only 2 trips touched it. `UNION` removes duplicate rows, and a round trip is exactly the case where the start row and the end row are identical.
- **Keep the `id`.** The `id` is what makes the deduplication right. Without it, `SELECT start_station UNION SELECT end_station` would collapse every visit to a station into one row, and each station would count 1.
- **`HAVING`, not `WHERE`.** The count exists only after grouping, so the "at least 3" test belongs in `HAVING`. `ORDER BY` may use the alias `trips`.
- **Another way.** `COUNT(*)` over the union equals, per station, the starts plus the ends that are not round trips: `UNION ALL` of `start_station` with `end_station` from the trips where `start_station <> end_station`. Both give the same rows.
- **`minutes` is noise.** The ride's length plays no part; it is there because real tables carry columns a query does not need.

## Cost

The union sorts or hashes about `2n` rows to drop duplicates, and the grouping sorts them by station: about `O(n log n)` for `n` trips. The final sort only sees the stations that passed `HAVING`.
