A neighbourhood bakery keeps its customers in `customers` and every order in `orders`. Totals are stored in cents, so an order of 100.00 has `total_cents = 10000`.

For the big cakes, the owner wants to call back everyone who placed a **big order**: an order with a total of at least 100.00. She needs each such order with the customer's name and city.

This is an **SQL problem**: write one `SELECT` statement (SQLite 3.53, as bundled). The tables are in the schema panel.

## Tables

- `customers(id, name, city)`: `id` is the primary key.
- `orders(id, customer_id, placed, total_cents)`: `id` is the primary key; `customer_id` refers to `customers(id)`; `placed` is the order's date, written `YYYY-MM-DD`; `total_cents` is the order's total in cents.

Customer ids and order ids are separate numberings: an order and a customer can have the same `id` without being related.

## Result

Return one row per order with `total_cents` of 10000 or more, with four columns:

| column | value |
| --- | --- |
| `order_id` | the order's `id` |
| `name` | the name of the customer who placed it |
| `city` | that customer's city |
| `total_cents` | the order's total in cents |

Order the rows by `total_cents`, largest first, then by `order_id`.

### Example 1

`customers`:

| id | name | city |
| --- | --- | --- |
| 1 | Ana | Porto |
| 2 | Bruno | Braga |
| 3 | Clara | Faro |

`orders`:

| id | customer_id | placed | total_cents |
| --- | --- | --- | --- |
| 1 | 2 | 2026-03-01 | 12500 |
| 2 | 3 | 2026-03-01 | 9999 |
| 3 | 1 | 2026-03-02 | 10000 |
| 4 | 2 | 2026-03-03 | 2500 |

Result:

| order_id | name | city | total_cents |
| --- | --- | --- | --- |
| 1 | Bruno | Braga | 12500 |
| 3 | Ana | Porto | 10000 |

Order 1 belongs to customer 2 (Bruno), not to customer 1. Order 3 is exactly 100.00, so it counts; order 2 falls one cent short.

### Example 2

`customers`:

| id | name | city |
| --- | --- | --- |
| 5 | Elena | Evora |

`orders`:

| id | customer_id | placed | total_cents |
| --- | --- | --- | --- |
| 7 | 5 | 2026-04-10 | 800 |

Result: no rows (only the columns `order_id`, `name`, `city` and `total_cents`).

## Constraints

- 1 ≤ customers ≤ 200, 0 ≤ orders ≤ 2000.
- Customer ids are distinct, and so are order ids, each from 1 to 10⁶.
- Every order's `customer_id` is the id of a customer.
- A name or a city is one capitalised word; two customers may share a name.
- `placed` is a valid date of 2026; 1 ≤ `total_cents` ≤ 10⁷.
