Before comparing anything, clean each name. Which two SQLite functions remove the surrounding spaces and fold the letter case?
---
`lower(trim(origin))` and `lower(trim(dest))` are the clean names; drop the rows where they are equal. Lima-Quito and Quito-Lima still look different, though: how can you write every pair in one fixed order?
---
With two or more arguments, `min(x, y)` and `max(x, y)` are ordinary functions (not aggregates) that return the smaller and the larger value. Select `min(...) AS city_a` and `max(...) AS city_b` of the two clean names, add `DISTINCT`, and finish with `ORDER BY city_a, city_b`.
