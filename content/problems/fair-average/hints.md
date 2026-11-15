One row per game means `GROUP BY game`. The question is how the aggregate functions treat a `NULL` in `stars`. What do `COUNT(*)`, `COUNT(stars)` and `AVG(stars)` each do with it?
---
`COUNT(stars)` counts only the non-`NULL` values, and `AVG(stars)` averages only them, which is exactly the fair average. When a group has no non-`NULL` value at all, `AVG` returns `NULL` while `COUNT(stars)` returns 0.
---
`SELECT game, COUNT(stars) AS rated, ROUND(AVG(stars), 2) AS avg_stars FROM reviews GROUP BY game ORDER BY game`. Do not turn the `NULL`s into 0 with `COALESCE`: that counts a comment as a 0-star review.
