After sorting the bookings by start, which bookings can overlap the current combined range?
---
Only the next ones in sorted order: if the next booking starts after the current range ends, no later booking can touch the current range either.
---
Sort by start. Keep the last combined range; for each booking, if its start ≤ the range's end, extend the end to `max(end, booking end)`; otherwise start a new range.
