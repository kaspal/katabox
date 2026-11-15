A credit union keeps every transaction in `transactions`: the account, the day and the amount in cents, positive for a deposit and negative for a withdrawal. Every account starts at a balance of 0.

For the members' statements, the clerk wants each transaction printed next to the account's **balance right after it**: the sum of that account's amounts up to and including it. Transactions are applied in order of day; when an account has several on the same day, the one with the smaller `id` comes first.

This is an **SQL problem**: write one `SELECT` statement (SQLite 3.53, as bundled). The tables are in the schema panel.

## Tables

- `transactions(id, account, day, amount_cents)`: `id` is the primary key; `account` is the account's code; `day` is written `YYYY-MM-DD`; `amount_cents` is the amount in cents, positive for deposits and negative for withdrawals.

## Result

Return one row per transaction, with four columns:

| column | value |
| --- | --- |
| `account` | the account's code |
| `day` | the transaction's day |
| `amount_cents` | the transaction's amount |
| `balance_cents` | the account's balance after this transaction |

Order the rows by `account`, then by `day`, then by the transaction's `id`.

### Example 1

`transactions`:

| id | account | day | amount_cents |
| --- | --- | --- | --- |
| 1 | AC001 | 2026-02-01 | 10000 |
| 2 | AC001 | 2026-02-03 | -2500 |
| 3 | AC002 | 2026-02-01 | 500 |
| 4 | AC001 | 2026-02-03 | -1000 |
| 5 | AC002 | 2026-02-02 | 700 |

Result:

| account | day | amount_cents | balance_cents |
| --- | --- | --- | --- |
| AC001 | 2026-02-01 | 10000 | 10000 |
| AC001 | 2026-02-03 | -2500 | 7500 |
| AC001 | 2026-02-03 | -1000 | 6500 |
| AC002 | 2026-02-01 | 500 | 500 |
| AC002 | 2026-02-02 | 700 | 1200 |

AC001 has two withdrawals on 2026-02-03: transaction 2 comes first and leaves 7500, then transaction 4 leaves 6500. AC002 keeps its own balance, from 0.

### Example 2

`transactions` is empty.

Result: empty (no rows).

## Constraints

- 0 ≤ transactions ≤ 3000.
- Ids are distinct, from 1 to 10⁶; they need not follow the order of the days.
- An account is `AC` and three digits.
- Every `day` is a valid date of 2026.
- `amount_cents` is not zero and −10⁶ ≤ `amount_cents` ≤ 10⁶; a balance may go below zero.
