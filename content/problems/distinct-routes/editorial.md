## Idea

Three steps, all on the same table:

1. **Clean** each name: `trim` removes the spaces around it and `lower` folds the case, so every spelling of a city becomes one value.
2. **Order the pair**: the smaller clean name goes in `city_a` and the larger in `city_b`, so both directions of a route give the same row.
3. **Drop the duplicates** with `DISTINCT`, after leaving out the flights whose two cities are the same.

## The query

```sql
SELECT DISTINCT
       min(lower(trim(origin)), lower(trim(dest))) AS city_a,
       max(lower(trim(origin)), lower(trim(dest))) AS city_b
FROM flights
WHERE lower(trim(origin)) <> lower(trim(dest))
ORDER BY city_a, city_b;
```

## Details that matter

- **One row per route, not per direction.** Cleaning the names and selecting `DISTINCT origin, dest` still returns `lima, quito` and `quito, lima` as two rows. The pair must be put in a fixed order first; `min` and `max` with two arguments do exactly that.
- **Two-argument `min` and `max` are not aggregates.** With a single argument they collapse the whole table into one row; with two or more they work row by row and return the smallest or largest argument. That is why the query needs no `GROUP BY`.
- **Clean before every comparison.** The filter must compare the clean names too: `origin <> dest` keeps `'Cali'` to `'cali'` and `'Arica'` to `'  Arica'`, which are flights from a city to itself. Forgetting `trim` makes `' cusco'` a different city from `'cusco'`, and since a space sorts before every letter it also lands in the wrong place.
- **Lowercase in the result.** `DISTINCT` and the sort work on the values you output, so the output must be the clean, lowercase names, not the original spelling of one of the flights.
- **No `NULL`s.** Both columns are `NOT NULL`, so no route is lost to an unknown city.

## Cost

One pass over the flights to clean the names, then SQLite removes the duplicates and sorts the routes with a temporary B-tree: about n log n steps for n flights.
