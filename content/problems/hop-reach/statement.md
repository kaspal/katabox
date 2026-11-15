A ferry company runs boats between the piers of a large lake. Each row of `routes` is a direct, **one-way** crossing from `from_pier` to `to_pier`; the way back, if there is one, is a separate row. Routes can form loops, including loops that lead back to the start.

A day ticket bought at the pier named `Central` is valid for at most **3 crossings**. The company wants to print every other pier a passenger can reach from `Central` with that ticket, together with the fewest crossings needed to get there.

This is an **SQL problem**: write one `SELECT` statement (SQLite 3.53, as bundled). The tables are in the schema panel. A `WITH RECURSIVE` query counts as one `SELECT` statement.

## Tables

- `routes(id, from_pier, to_pier)`: `id` is the primary key; each row is a crossing from `from_pier` to `to_pier`, in that direction only.

Piers exist only as names in `routes`. The starting pier is named exactly `Central`; it may have no routes at all.

## Result

Return one row per pier, other than `Central`, that can be reached from `Central` in at most 3 crossings, with two columns:

| column | value |
| --- | --- |
| `pier` | the pier's name |
| `hops` | the fewest crossings needed to reach it from `Central` (1 to 3) |

`Central` itself is never listed, even when a route leads back to it. Order the rows by `hops`, smallest first, then by `pier`.

### Example 1

`routes`:

| id | from_pier | to_pier |
| --- | --- | --- |
| 1 | Central | Harbour |
| 2 | Harbour | Island |
| 3 | Island | Lighthouse |
| 4 | Lighthouse | Cove |
| 5 | Central | Island |
| 6 | Island | Central |

Result:

| pier | hops |
| --- | --- |
| Harbour | 1 |
| Island | 1 |
| Lighthouse | 2 |
| Cove | 3 |

Harbour and Island are one crossing from Central; Island can also be reached through Harbour in two, but the fewest crossings count. Lighthouse is two crossings away through Island and Cove three. The route from Island back to Central does not put Central on the list.

### Example 2

`routes`:

| id | from_pier | to_pier |
| --- | --- | --- |
| 7 | Market | Central |

Result: empty (no rows). The only route arrives at Central, and none leaves it.

## Constraints

- 1 ≤ routes ≤ 400.
- Ids are distinct, from 1 to 10⁶.
- A pier's name is one capitalised word of 3 to 15 letters.
- A route joins two different piers, and each one-way pair (`from_pier`, `to_pier`) appears at most once.
