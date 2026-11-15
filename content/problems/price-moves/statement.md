An investment club tracks a few listed companies. The table `prices` holds one closing price per company (its `ticker`) and trading day, in cents. Markets close on weekends and holidays, and the club did not record every day, so a company's trading days are not always consecutive calendar days, and two companies may trade on different days.

For each company and each of its trading days **except its first**, the club wants that day's close next to the **previous close**: the same company's close on its most recent earlier day in the table, together with the change in percent.

This is an **SQL problem**: write one `SELECT` statement (SQLite 3.53, as bundled). The tables are in the schema panel.

## Tables

- `prices(id, ticker, day, close_cents)`: `id` is the primary key; `day` is a date written `YYYY-MM-DD`; `close_cents` is the closing price in cents. There is at most one row per `ticker` and `day`, and the rows are in no particular order.

## Result

Return one row per company and trading day that has a previous day, with five columns:

| column | value |
| --- | --- |
| `ticker` | the company |
| `day` | the trading day |
| `close_cents` | the close on `day` |
| `prev_cents` | the same company's close on its previous trading day |
| `change_pct` | `100 × (close_cents − prev_cents) / prev_cents`, rounded to 2 decimals |

Compute `change_pct` with real (not integer) division and round it with SQLite's `ROUND(x, 2)`, which rounds halves away from zero. A value within 0.001 of the expected one is accepted.

Order the rows by `ticker`, then by `day`.

### Example 1

`prices`:

| id | ticker | day | close_cents |
| --- | --- | --- | --- |
| 1 | ACME | 2026-03-02 | 1000 |
| 2 | ACME | 2026-03-03 | 1050 |
| 3 | ACME | 2026-03-04 | 1029 |
| 4 | BOLT | 2026-03-02 | 400 |
| 5 | BOLT | 2026-03-04 | 300 |

Result:

| ticker | day | close_cents | prev_cents | change_pct |
| --- | --- | --- | --- | --- |
| ACME | 2026-03-03 | 1050 | 1000 | 5.0 |
| ACME | 2026-03-04 | 1029 | 1050 | -2.0 |
| BOLT | 2026-03-04 | 300 | 400 | -25.0 |

ACME goes from 1000 to 1050 (+5%) and then to 1029 (−2%). BOLT has no row for 2026-03-03, so its close on 2026-03-04 is compared with 2026-03-02: from 400 to 300 is −25%. Each company's first day has no previous close and is not listed.

### Example 2

`prices`:

| id | ticker | day | close_cents |
| --- | --- | --- | --- |
| 6 | CORA | 2026-03-02 | 777 |

Result: empty (no rows). CORA has only one day, so it has no previous close.

## Constraints

- 0 ≤ rows in `prices` ≤ 3000.
- Ids are distinct, from 1 to 10⁶.
- A ticker is 3 or 4 capital letters.
- Every `day` is a valid date of 2026, with at most one row per ticker and day.
- 1 ≤ `close_cents` ≤ 10⁷.
