## Idea

SQL's aggregate functions already skip `NULL`, which is exactly the rule the café asks for:

1. **Group** the reviews by game.
2. **Count and average** the stars, letting `COUNT(stars)` and `AVG(stars)` ignore the reviews without them.
3. **Round** the average and order by game.

## The query

```sql
SELECT game,
       COUNT(stars) AS rated,
       ROUND(AVG(stars), 2) AS avg_stars
FROM reviews
GROUP BY game
ORDER BY game;
```

## Details that matter

- **A missing score is not a zero.** `AVG(COALESCE(stars, 0))` treats a comment as a 0-star review: in Example 1 Hive drops from 4.5 to 3.0 and Azul shows 0.0 instead of `NULL`. Leaving `stars` as it is lets `AVG` skip the `NULL`s.
- **`COUNT(stars)` versus `COUNT(*)`.** `COUNT(*)` counts rows, so it would give Hive 3; `COUNT(column)` counts only the non-`NULL` values, which is what `rated` means.
- **All-`NULL` groups.** A game is listed because it has rows, even if none has stars. Then `COUNT(stars)` is 0 and `AVG(stars)` is `NULL`, and `ROUND(NULL, 2)` stays `NULL`, as required, with no `CASE` needed.
- **Rounding.** `AVG` returns a real number; `ROUND(x, 2)` keeps two decimals and rounds a half away from zero (3.125 becomes 3.13). An average of 2 comes out as the real `2.0`, as in Example 2.
- **Order.** Games are distinct after grouping, so `ORDER BY game` is a full order.

## Cost

One grouping pass: SQLite sorts the reviews by game (or uses a temporary index) and keeps a running count and sum per group, about `O(n log n)` for `n` reviews.
