## Idea

Each booking becomes one row after three lookups:

1. **Join** the booking to its flight, for the flight number, the two airport codes and the departure time.
2. **Join `airports` twice**, once per end of the flight, to turn both codes into cities.
3. **Order** by passenger, then by departure time.

## The query

```sql
SELECT b.passenger, f.flight_no, src.city AS from_city, dst.city AS to_city
FROM bookings b
JOIN flights f ON f.id = b.flight_id
JOIN airports src ON src.code = f.from_code
JOIN airports dst ON dst.code = f.to_code
ORDER BY b.passenger, f.departs, b.id;
```

## Details that matter

- **Two copies of `airports`.** A single joined row of `airports` is one airport, so it cannot be both ends of a flight. Joining it once on `from_code` and selecting its city for both columns makes every leg land where it took off; each end needs its own alias (`src` and `dst`) and its own `ON` condition.
- **Aliases name the columns.** Both copies have a column called `city`, so `AS from_city` and `AS to_city` are needed to give the result its column names.
- **Times sort as text.** `YYYY-MM-DD HH:MM` with zero padding puts earlier times first when compared as strings, so `ORDER BY f.departs` is the departure order without any date function. `departs` is not in the result, which `ORDER BY` allows.
- **A full order.** Departure times are distinct, but a passenger may book the same flight twice; the booking's `id` breaks that last tie.
- **Inner joins are enough.** Every booking has a flight and every flight has both airports, so no row is lost. Flights nobody booked give no row, as Example 2 shows.

## Cost

Each booking needs three primary-key lookups (its flight, then two airports), so the joins take about one step per booking; the sort of the result costs `O(b log b)` for `b` bookings.
