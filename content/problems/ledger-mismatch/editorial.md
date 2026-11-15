## Idea

A mismatch can sit on either side, so the query has to look from both tables at once:

1. **Full outer join** `bank` and `books` on `ref`: every reference from either table gets one row, with `NULL` on the side where it is missing.
2. **Filter** to the rows that do not agree: missing on one side, or different amounts.
3. **Take the reference** from whichever side has it, and order by it.

## The query

```sql
SELECT COALESCE(b.ref, k.ref) AS ref,
       b.amount_cents AS bank_cents,
       k.amount_cents AS books_cents
FROM bank b
FULL OUTER JOIN books k ON k.ref = b.ref
WHERE b.ref IS NULL OR k.ref IS NULL OR b.amount_cents <> k.amount_cents
ORDER BY COALESCE(b.ref, k.ref);
```

## Details that matter

- **Looking from one side only.** A `LEFT JOIN` from `bank` finds payments missing from the books and amounts that differ, but a payment that is only in the books has no `bank` row to start from, so it is silently lost. With an empty `bank` table such a query returns nothing at all. The full outer join keeps the unmatched rows of both tables.
- **`NULL` never differs.** When one side is missing, `b.amount_cents <> k.amount_cents` compares with `NULL` and is not true, so on its own it would drop exactly the one-sided references. The explicit `b.ref IS NULL OR k.ref IS NULL` tests keep them. Testing `ref` is safe because it is `NOT NULL` in both tables, so a `NULL` there can only come from the join.
- **The reference column.** On a one-sided row, `b.ref` or `k.ref` is `NULL`; `COALESCE(b.ref, k.ref)` picks the one that exists, for the result and for the order.
- **Match by `ref`, not `id`.** The two tables number their rows independently, as Example 2 shows.
- **Without `FULL OUTER JOIN`.** The same result comes from a `LEFT JOIN` from `bank` (as above) combined by `UNION ALL` with the `books` rows whose `ref` is `NOT IN (SELECT ref FROM bank)`, with `NULL` as their `bank_cents`, ordered by `ref` at the end. Because each ref appears at most once per table, no reference can come out twice.

## Cost

With an index on `ref` on either side, each row of one table finds its partner in about `O(log n)`, and the unmatched rows of the other side are found the same way; then the mismatches are sorted by `ref`. Without an index SQLite builds a temporary one, which costs about the same.
