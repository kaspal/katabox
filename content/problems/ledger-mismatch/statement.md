At the end of the month a shop checks its books against the bank. The bank's statement is in `bank` and the shop's own records are in `books`; both list payments by their payment reference, with the amount in cents.

The accountant wants every reference where the two **do not agree**: a payment the bank shows but the books miss, one in the books that never reached the bank, or one in both with a different amount. References that match on both sides, with the same amount, are fine and stay off the list.

This is an **SQL problem**: write one `SELECT` statement (SQLite 3.53, as bundled). The tables are in the schema panel.

## Tables

- `bank(id, ref, amount_cents)`: `id` is the primary key; `ref` is the payment reference; `amount_cents` is the amount the bank recorded, in cents.
- `books(id, ref, amount_cents)`: the same columns, as the shop recorded them.

A reference appears at most once in each table. The ids of the two tables are unrelated: match the rows by `ref`.

## Result

Return one row per reference that does not agree, with three columns:

| column | value |
| --- | --- |
| `ref` | the payment reference |
| `bank_cents` | the amount in `bank`, or `NULL` if the reference is not there |
| `books_cents` | the amount in `books`, or `NULL` if the reference is not there |

Order the rows by `ref`.

### Example 1

`bank`:

| id | ref | amount_cents |
| --- | --- | --- |
| 1 | P0001 | 1500 |
| 2 | P0002 | 2000 |
| 3 | P0004 | 700 |

`books`:

| id | ref | amount_cents |
| --- | --- | --- |
| 1 | P0001 | 1500 |
| 2 | P0002 | 2500 |
| 3 | P0003 | 900 |

Result:

| ref | bank_cents | books_cents |
| --- | --- | --- |
| P0002 | 2000 | 2500 |
| P0003 | NULL | 900 |
| P0004 | 700 | NULL |

P0001 agrees and is left out. P0002 is in both with different amounts, P0003 is only in the books and P0004 only at the bank.

### Example 2

`bank`:

| id | ref | amount_cents |
| --- | --- | --- |
| 1 | P0100 | 4200 |

`books`:

| id | ref | amount_cents |
| --- | --- | --- |
| 9 | P0100 | 4200 |

Result: empty (no rows). The only payment agrees, even though its ids differ.

## Constraints

- 0 ≤ rows in `bank` ≤ 2000 and 0 ≤ rows in `books` ≤ 2000, with at least one row in all. Either table may be empty.
- Within each table, ids are distinct, from 1 to 10⁶, and refs are distinct; a ref is `P` and four digits.
- −10⁷ ≤ `amount_cents` ≤ 10⁷; an amount may be zero or negative (a refund).
- SQLite 3.53 supports `FULL OUTER JOIN`, but any correct `SELECT` is accepted.
