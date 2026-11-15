An orchard writes every invoice into `sales`: the day it was sold and the amount in cents. For the yearly report the owner wants one line per year, with that year's takings split into its four quarters side by side.

The quarters are the usual calendar ones: Q1 is January to March, Q2 April to June, Q3 July to September and Q4 October to December. A quarter with no sales shows 0, and a year with no sales at all does not appear.

This is an **SQL problem**: write one `SELECT` statement (SQLite 3.53, as bundled). The tables are in the schema panel.

## Tables

- `sales(id, sold_on, amount_cents)`: `id` is the primary key; `sold_on` is the day of the sale, written `YYYY-MM-DD`; `amount_cents` is the invoice's amount, in cents.

## Result

Return one row per year that has at least one sale, with five columns:

| column | value |
| --- | --- |
| `year` | the year, as an integer |
| `q1` | the sum of that year's amounts sold in January, February or March, or 0 |
| `q2` | the same for April, May and June |
| `q3` | the same for July, August and September |
| `q4` | the same for October, November and December |

Order the rows by `year`.

### Example 1

`sales`:

| id | sold_on | amount_cents |
| --- | --- | --- |
| 1 | 2025-01-15 | 1200 |
| 2 | 2025-03-31 | 800 |
| 3 | 2025-04-01 | 500 |
| 4 | 2025-12-24 | 3000 |
| 5 | 2026-07-04 | 2200 |

Result:

| year | q1 | q2 | q3 | q4 |
| --- | --- | --- | --- | --- |
| 2025 | 2000 | 500 | 0 | 3000 |
| 2026 | 0 | 0 | 2200 | 0 |

In 2025, the January and March sales make Q1 (1200 + 800), April's is Q2 and December's is Q4; nothing was sold in Q3. 2026 has only a July sale, in Q3.

### Example 2

`sales` is empty.

Result: empty (no rows).

## Constraints

- 0 ≤ sales ≤ 3000.
- Ids are distinct, from 1 to 10⁶.
- Every `sold_on` is a valid date from 2020 to 2026.
- 1 ≤ `amount_cents` ≤ 10⁶.
