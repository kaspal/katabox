A city bike-share scheme records every ride in `trips`: the dock station where the bike was taken, the station where it was left, and how long the ride took.

The planners want to know which stations see the most traffic. A trip touches its start station and its end station, so it counts once for each. A **round trip**, which starts and ends at the same station, touches only one station and counts **once** there. Quiet stations, with fewer than 3 trips, are left out.

This is an **SQL problem**: write one `SELECT` statement (SQLite 3.53, as bundled). The tables are in the schema panel.

## Tables

- `trips(id, start_station, end_station, minutes)`: `id` is the primary key; `start_station` and `end_station` are station names; `minutes` is the ride's length (not needed here).

## Result

Return one row per station with at least 3 trips, with two columns:

| column | value |
| --- | --- |
| `station` | the station's name |
| `trips` | the number of trips that start or end there, a round trip counted once |

Order the rows by `trips`, largest first, then by `station`.

### Example 1

`trips`:

| id | start_station | end_station | minutes |
| --- | --- | --- | --- |
| 1 | Market | Park | 12 |
| 2 | Park | Market | 15 |
| 3 | Museum | Museum | 40 |
| 4 | Market | Museum | 9 |
| 5 | Museum | Park | 20 |

Result:

| station | trips |
| --- | --- |
| Market | 3 |
| Museum | 3 |
| Park | 3 |

Museum has trips 3, 4 and 5; trip 3 is a round trip and counts once. Market (trips 1, 2, 4) and Park (trips 1, 2, 5) also have 3, so the three stations tie and are ordered by name.

### Example 2

`trips`:

| id | start_station | end_station | minutes |
| --- | --- | --- | --- |
| 6 | Bridge | Bridge | 30 |
| 7 | Bridge | Bridge | 25 |

Result: empty (no rows). Bridge has only 2 trips.

## Constraints

- 0 ≤ trips ≤ 3000.
- Ids are distinct, from 1 to 10⁶.
- A station name is one capitalised word of 3 to 12 letters.
- 1 ≤ `minutes` ≤ 600.
