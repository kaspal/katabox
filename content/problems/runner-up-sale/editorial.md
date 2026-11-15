## Idea

"Second-highest distinct amount" is easier to compute as "the highest amount below the top":

1. **List the regions** once each, so every region gets a row.
2. **Find the top** amount of the region.
3. **Take the largest amount strictly below it**; if there is none, the answer is `NULL` by itself.

## The query

```sql
SELECT r.region,
       (SELECT MAX(s.amount_cents)
        FROM sales s
        WHERE s.region = r.region
          AND s.amount_cents < (SELECT MAX(t.amount_cents) FROM sales t WHERE t.region = r.region)
       ) AS runner_up_cents
FROM (SELECT DISTINCT region FROM sales) r
ORDER BY r.region;
```

## Details that matter

- **The second row is not the runner-up.** `ORDER BY s.amount_cents DESC LIMIT 1 OFFSET 1` takes the second *sale*. When two sales tie for the top, that second sale has the top amount, so the top amount becomes its own runner-up: South in Example 1 would get 3000 instead of 1000. Comparing with `<` against the top skips every copy of it. (`SELECT DISTINCT amount_cents ... LIMIT 1 OFFSET 1` would also work.)
- **`NULL` comes for free.** An aggregate like `MAX` over no rows returns one row holding `NULL`, so a region with a single distinct amount gets `NULL` without a `CASE`. A `LIMIT`/`OFFSET` subquery that finds no row also yields `NULL`, but a filter like `WHERE amount_cents < top` in the outer query would drop the region instead.
- **Every region appears.** Grouping only the sales below the top would lose regions such as East in Example 2. Driving the query from `SELECT DISTINCT region` keeps them.
- **With window functions.** `DENSE_RANK() OVER (PARTITION BY region ORDER BY amount_cents DESC)` numbers the distinct amounts 1, 2, ... in each region, ties sharing a number. The runner-up is then `MAX(CASE WHEN rnk = 2 THEN amount_cents END)` grouped by region, which is `NULL` where no amount has rank 2.

## Cost

For each of the `r` regions, the two subqueries scan the sales: about `O(r · n)` steps for `n` sales, which is small here since there are only a few regions. An index on `(region, amount_cents)` would let each `MAX` be found in about `O(log n)`.
