An ordinary `JOIN` on `ref` only keeps references that are in both tables, and two of the three kinds of mismatch are in just one. Which join keeps the unmatched rows of *both* sides?
---
`bank b FULL OUTER JOIN books k ON k.ref = b.ref` keeps every reference from either side; the side that lacks it is all `NULL`. (Without it: a `LEFT JOIN` from `bank`, plus a `UNION ALL` of the `books` rows whose `ref` is not in `bank`.) The reference itself then comes from whichever side has it: `COALESCE(b.ref, k.ref)`.
---
Keep a row when `b.ref IS NULL OR k.ref IS NULL OR b.amount_cents <> k.amount_cents`; the first two tests matter because `<>` with a `NULL` is never true. Select `COALESCE(b.ref, k.ref) AS ref, b.amount_cents AS bank_cents, k.amount_cents AS books_cents` and order by `COALESCE(b.ref, k.ref)`.
