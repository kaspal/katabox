A food truck logs every order it sells in `orders`: when it was placed, down to the minute, and its total in cents.

At the end of the season the owner wants a **daily summary**: for each day the truck took orders, how many orders there were and how much they brought in. Days without any order are not part of the summary.

This is an **SQL problem**: write one `SELECT` statement (SQLite 3.53, as bundled). The tables are in the schema panel.

## Tables

- `orders(id, placed, total_cents)`: `id` is the primary key; `placed` is the time the order was placed, written `YYYY-MM-DD HH:MM`; `total_cents` is the order's total in cents.

The day of an order is the first 10 characters of `placed` (`YYYY-MM-DD`).

## Result

Return one row per day with at least one order, with three columns:

| column | value |
| --- | --- |
| `day` | the day, written `YYYY-MM-DD` |
| `orders` | the number of orders placed that day |
| `revenue_cents` | the sum of `total_cents` of those orders |

Order the rows by `day`, earliest first.

### Example 1

`orders`:

| id | placed | total_cents |
| --- | --- | --- |
| 1 | 2026-05-01 12:05 | 850 |
| 2 | 2026-05-01 12:05 | 1200 |
| 3 | 2026-05-01 19:40 | 640 |
| 4 | 2026-05-03 13:10 | 990 |

Result:

| day | orders | revenue_cents |
| --- | --- | --- |
| 2026-05-01 | 3 | 2690 |
| 2026-05-03 | 1 | 990 |

The three orders of May 1 fall on the same day even though they were placed at two different times, so they form one row: 850 + 1200 + 640 = 2690. May 2 had no orders and does not appear.

### Example 2

`orders`: empty (no rows).

Result: empty (no rows).

## Constraints

- 0 ≤ orders ≤ 3000.
- Ids are distinct, from 1 to 10⁶.
- `placed` is a valid time of 2026, written `YYYY-MM-DD HH:MM`; several orders may share the same minute.
- 1 ≤ `total_cents` ≤ 10⁶.
