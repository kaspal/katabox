A home-goods shop is planning a "gifts from 20 to 50" shelf. Every product is in `products`, with its price stored in cents so that no rounding ever creeps in, and a flag that says whether it is on the shelves right now.

The shop wants the products for the shelf: **in stock** (`in_stock = 1`) and priced **from 2000 to 4999 cents, both ends included** (20.00 to 49.99). A product at exactly 20.00 or exactly 49.99 belongs on the shelf; one at 50.00 does not.

This is an **SQL problem**: write one `SELECT` statement (SQLite 3.53, as bundled). The tables are in the schema panel.

## Tables

- `products(id, name, price_cents, in_stock)`: `id` is the primary key; `price_cents` is the price in cents (`2599` means 25.99); `in_stock` is `1` when the product is available and `0` when it is not.

## Result

Return one row per product for the shelf, with two columns:

| column | value |
| --- | --- |
| `name` | the product's name |
| `price_cents` | its price in cents |

Order the rows by `price_cents`, highest first, then by `name`, then by the product's `id`.

### Example 1

`products`:

| id | name | price_cents | in_stock |
| --- | --- | --- | --- |
| 1 | lamp | 2000 | 1 |
| 2 | rug | 4999 | 1 |
| 3 | vase | 5000 | 1 |
| 4 | mug | 1999 | 1 |
| 5 | clock | 3500 | 0 |
| 6 | tray | 3500 | 1 |

Result:

| name | price_cents |
| --- | --- |
| rug | 4999 |
| tray | 3500 |
| lamp | 2000 |

The lamp and the rug sit exactly on the two ends of the band, so both are in. The vase and the mug are one cent outside it, and the clock is out of stock.

### Example 2

`products`:

| id | name | price_cents | in_stock |
| --- | --- | --- | --- |
| 1 | pen | 2500 | 1 |
| 2 | ink | 2500 | 1 |
| 3 | pad | 2500 | 1 |

Result:

| name | price_cents |
| --- | --- |
| ink | 2500 |
| pad | 2500 |
| pen | 2500 |

## Constraints

- 1 ≤ products ≤ 500; the ids are distinct, from 1 to 10⁶.
- A name is one or two lowercase words; two products may have the same name.
- 1 ≤ `price_cents` ≤ 10⁶, a whole number.
- `in_stock` is `0` or `1`.
