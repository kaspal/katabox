## Idea

A share compares a part with the whole, so the query needs both at once:

1. **Group** the sales by category to get each part, `SUM(amount_cents)`.
2. **Get the whole** with a scalar subquery over the entire table.
3. **Divide** as real numbers, round, and order by the rounded share.

## The query

```sql
SELECT category,
       SUM(amount_cents) AS total_cents,
       ROUND(100.0 * SUM(amount_cents) / (SELECT SUM(amount_cents) FROM sales), 1) AS share_pct
FROM sales
GROUP BY category
ORDER BY share_pct DESC, category;
```

## Details that matter

- **Integer division.** Both sums are integers, and SQLite divides two integers as integers, dropping the remainder. `100 * SUM(amount_cents) / total` therefore yields a whole number before `ROUND` ever sees it: a category with 1 of 3 cents gets 33.0 instead of 33.3. Writing `100.0` makes the product real, so the division keeps its decimals. Multiplying first matters too: `SUM(amount_cents) / total * 100` with integers is 0 for every category but a lone one.
- **The grand total.** Inside a grouped query, `SUM(amount_cents)` is the group's sum. The subquery `(SELECT SUM(amount_cents) FROM sales)` is not correlated, so SQLite computes it once. A window function, `SUM(SUM(amount_cents)) OVER ()`, gives the same total.
- **Order by the rounded share.** The order uses the alias, which is the rounded value, so two categories whose exact shares differ slightly but round to the same number are ordered by `category`. Ordering by the unrounded share could put them the other way round.
- **No division by zero.** Every amount is at least 1 and the table has at least one row, so the total is positive.

## Cost

One grouping pass over the `n` sales lines, sorted by category, plus one pass for the grand total: about `O(n log n)`. The final sort only sees one row per category.
