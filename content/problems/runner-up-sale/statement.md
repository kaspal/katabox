A furniture chain records every sale in `sales`, with the region where it happened and the amount in cents. Each quarter it gives a prize to the biggest sale of every region, and a smaller one to the **runner-up**.

The runner-up of a region is its second-highest *distinct* amount: the largest amount that is strictly below the region's top amount. Two sales tied for the top are both the top, so neither one is the runner-up. A region whose sales all have the same amount has no runner-up.

This is an **SQL problem**: write one `SELECT` statement (SQLite 3.53, as bundled). The tables are in the schema panel.

## Tables

- `sales(id, region, amount_cents)`: `id` is the primary key; `region` is the region's name; `amount_cents` is the sale's amount, in cents.

## Result

Return one row for every region that has at least one sale, with two columns:

| column | value |
| --- | --- |
| `region` | the region's name |
| `runner_up_cents` | the region's second-highest distinct amount, or `NULL` if it has fewer than two distinct amounts |

Order the rows by `region`.

### Example 1

`sales`:

| id | region | amount_cents |
| --- | --- | --- |
| 1 | North | 5000 |
| 2 | North | 9000 |
| 3 | North | 7000 |
| 4 | South | 3000 |
| 5 | South | 3000 |
| 6 | South | 1000 |

Result:

| region | runner_up_cents |
| --- | --- |
| North | 7000 |
| South | 1000 |

North's top amount is 9000, so its runner-up is 7000. In South two sales share the top amount, 3000, so the runner-up is the next amount down, 1000.

### Example 2

`sales`:

| id | region | amount_cents |
| --- | --- | --- |
| 7 | East | 2500 |
| 8 | East | 2500 |

Result:

| region | runner_up_cents |
| --- | --- |
| East | NULL |

East has two sales but only one distinct amount, so it has no runner-up; the region still appears.

## Constraints

- 1 ≤ sales ≤ 2000.
- Ids are distinct, from 1 to 10⁶.
- A region is one capitalised word of 3 to 12 letters.
- 1 ≤ `amount_cents` ≤ 10⁷.
