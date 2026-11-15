A small charter airline logs every flight by hand in `flights`. The clerks are not careful: the same city shows up as `'Lima'`, `'LIMA'` or `' lima '`, depending on who typed it.

The planning team wants the list of **distinct routes** the airline has flown. A route joins two different cities, whatever the direction: a flight from Lima to Quito and one from Quito to Lima are the same route. A flight that starts and ends in the same city is not a route.

This is an **SQL problem**: write one `SELECT` statement (SQLite 3.53, as bundled). The tables are in the schema panel.

## Tables

- `flights(id, origin, dest)`: `id` is the primary key; `origin` and `dest` are the city the flight left from and the city it landed in, as typed. Neither is ever `NULL`.

Two names are the same city when they are equal after removing the spaces around them and ignoring letter case: `' Cusco'`, `'cusco'` and `'CUSCO  '` are all `cusco`.

## Result

Return one row per route, with two columns:

| column | value |
| --- | --- |
| `city_a` | the city of the route that comes first alphabetically |
| `city_b` | the other city |

Write both cities in lowercase, without the surrounding spaces, so `city_a < city_b` in every row. Each route appears once, however many flights used it and in whichever direction. Order the rows by `city_a`, then by `city_b`.

### Example 1

`flights` (quotes show the spaces):

| id | origin | dest |
| --- | --- | --- |
| 1 | `'Lima'` | `'Quito'` |
| 2 | `'quito'` | `'LIMA'` |
| 3 | `' Cusco'` | `'Lima '` |
| 4 | `'Lima'` | `'Cusco'` |

Result:

| city_a | city_b |
| --- | --- |
| cusco | lima |
| lima | quito |

Flights 1 and 2 are the route lima-quito, flown both ways and typed differently. Flights 3 and 4 are both the route cusco-lima once the spaces and capitals are gone.

### Example 2

`flights`:

| id | origin | dest |
| --- | --- | --- |
| 5 | `'Cali'` | `'cali'` |
| 6 | `'Arica'` | `'  Arica'` |

Result: no rows (only the columns `city_a` and `city_b`). Both flights start and end in the same city.

## Constraints

- 1 ≤ flights ≤ 2000; the ids are distinct.
- A city name is 2 to 12 letters (`A-Z`, `a-z` only), with at most two spaces before it and at most two after it.
