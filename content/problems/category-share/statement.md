A stationery shop records each line of every sale in `sales`: the product's category and the amount, in cents.

The owner wants to see where the money comes from: for every category, its total sales and its **share** of all the shop's sales, as a percentage. The biggest categories go first.

This is an **SQL problem**: write one `SELECT` statement (SQLite 3.53, as bundled). The tables are in the schema panel.

## Tables

- `sales(id, category, amount_cents)`: `id` is the primary key; `category` is the product's category; `amount_cents` is the line's amount in cents.

## Result

Return one row per category, with three columns:

| column | value |
| --- | --- |
| `category` | the category |
| `total_cents` | the sum of `amount_cents` of its lines |
| `share_pct` | 100 × `total_cents` ÷ the sum of all `amount_cents` in the table, computed exactly (with decimals) and then rounded to 1 decimal with SQLite's `ROUND` |

Order the rows by `share_pct` (the rounded value), largest first, then by `category`. Values of `share_pct` within 0.001 of the expected ones are accepted.

### Example 1

`sales`:

| id | category | amount_cents |
| --- | --- | --- |
| 1 | pens | 300 |
| 2 | paper | 500 |
| 3 | pens | 200 |
| 4 | ink | 1000 |

Result:

| category | total_cents | share_pct |
| --- | --- | --- |
| ink | 1000 | 50.0 |
| paper | 500 | 25.0 |
| pens | 500 | 25.0 |

All sales add up to 2000 cents. Ink brings 1000 of them, so 50.0%; paper and pens bring 500 each, 25.0%, and are ordered by name.

### Example 2

`sales`:

| id | category | amount_cents |
| --- | --- | --- |
| 5 | cards | 999 |

Result:

| category | total_cents | share_pct |
| --- | --- | --- |
| cards | 999 | 100.0 |

## Constraints

- 1 ≤ sales ≤ 3000.
- Ids are distinct, from 1 to 10⁶.
- A category is 3 to 12 lowercase letters.
- 1 ≤ `amount_cents` ≤ 10⁶.
