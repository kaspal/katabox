## Idea

A balance after each transaction is a running sum, which is what window functions are for:

1. **Partition** the transactions by account, so each account's balance starts from 0.
2. **Order** each account's transactions by day, then id: the order they are applied in.
3. **Sum** from the first row of the partition up to the current row.

## The query

```sql
SELECT account, day, amount_cents,
       SUM(amount_cents) OVER (
         PARTITION BY account
         ORDER BY day, id
         ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
       ) AS balance_cents
FROM transactions
ORDER BY account, day, id;
```

## Details that matter

- **Same-day transactions.** With `OVER (PARTITION BY account ORDER BY day)` and no frame, SQLite uses the default frame `RANGE BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW`, which includes the current row's *peers*: every row with the same `day`. Both of AC001's withdrawals on 2026-02-03 in Example 1 would then show the day's closing balance, 6500, instead of 7500 and 6500. Adding `id` to the window's order removes the ties, and `ROWS` makes the frame stop exactly at the current row.
- **The window order is not the result order.** The `ORDER BY` inside `OVER` decides what each sum covers; the final `ORDER BY account, day, id` decides how the rows are listed. Here they agree, but you need both.
- **Ids are not chronological.** A transaction with a larger id can be on an earlier day, so ordering by `id` alone gives wrong balances; the day comes first.
- **Without window functions.** A correlated subquery also works: for each row `t`, sum the amounts of the same account with `day < t.day OR (day = t.day AND id <= t.id)`. It gives the same result but rescans the account for every row.

## Cost

SQLite sorts the `n` transactions by account, day and id once, then walks them keeping one running sum per partition: `O(n log n)` for the sort and `O(n)` for the sums. The correlated subquery is about `O(n²)` in the worst case.
