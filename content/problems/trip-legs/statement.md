A small airline stores its airports in `airports`, its scheduled flights in `flights` and every seat sold in `bookings`. A flight only knows the codes of the airports it leaves from and flies to; the city names live in `airports`.

For the travel summaries it sends out, the airline wants **every leg of every passenger's trip**: one row per booking, with the flight number and the cities it flies between, each passenger's legs listed in the order they depart.

This is an **SQL problem**: write one `SELECT` statement (SQLite 3.53, as bundled). The tables are in the schema panel.

## Tables

- `airports(code, city)`: `code` is the primary key, a three-letter airport code; `city` is the city it serves.
- `flights(id, flight_no, from_code, to_code, departs)`: `id` is the primary key; `from_code` (departure airport) and `to_code` (arrival airport) refer to `airports(code)`; `departs` is the departure time written `YYYY-MM-DD HH:MM`.
- `bookings(id, passenger, flight_id)`: `id` is the primary key; `passenger` is the traveller's name; `flight_id` refers to `flights(id)`.

## Result

Return one row per booking, with four columns:

| column | value |
| --- | --- |
| `passenger` | the passenger's name |
| `flight_no` | the flight's number |
| `from_city` | the city of the departure airport |
| `to_city` | the city of the arrival airport |

Order the rows by `passenger`, then by the flight's departure time (earliest first), then by the booking's `id`.

### Example 1

`airports`:

| code | city |
| --- | --- |
| LIM | Lima |
| CUZ | Cusco |
| UIO | Quito |

`flights`:

| id | flight_no | from_code | to_code | departs |
| --- | --- | --- | --- | --- |
| 1 | KB101 | LIM | CUZ | 2026-06-01 08:00 |
| 2 | KB102 | CUZ | LIM | 2026-06-05 18:30 |
| 3 | KB200 | LIM | UIO | 2026-06-02 07:15 |

`bookings`:

| id | passenger | flight_id |
| --- | --- | --- |
| 1 | bo | 3 |
| 2 | ana | 2 |
| 3 | ana | 1 |

Result:

| passenger | flight_no | from_city | to_city |
| --- | --- | --- | --- |
| ana | KB101 | Lima | Cusco |
| ana | KB102 | Cusco | Lima |
| bo | KB200 | Lima | Quito |

ana booked her return flight first, but KB101 departs on June 1 and KB102 on June 5, so her outbound leg comes first. Each flight's two airports give two different cities.

### Example 2

`airports`:

| code | city |
| --- | --- |
| LIM | Lima |
| CUZ | Cusco |

`flights`:

| id | flight_no | from_code | to_code | departs |
| --- | --- | --- | --- | --- |
| 4 | KB300 | CUZ | LIM | 2026-07-01 12:00 |

`bookings`: empty.

Result: empty (no rows). A flight nobody booked gives no row.

## Constraints

- 2 ≤ airports ≤ 30, 1 ≤ flights ≤ 300, 0 ≤ bookings ≤ 2000.
- Airport codes are distinct, three capital letters; a city is one capitalised word.
- Flight ids and booking ids are distinct, from 1 to 10⁶. A flight number is `KB` and three digits.
- A flight goes between two different existing airports. Departure times are distinct, valid times in 2026.
- A passenger's name is 1 to 10 lowercase letters; every booking is for an existing flight. A passenger may book the same flight more than once.
