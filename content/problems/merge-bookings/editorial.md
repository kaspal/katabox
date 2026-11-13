## Approach: sort by start, then sweep

Sort the bookings by start. Walk through them keeping the current combined range `[s, e]`:

- if the next booking starts at or before `e` (they share a moment, since both ends are included), extend `e = max(e, booking end)`;
- otherwise the current range is finished: output it and start a new one at this booking.

Output the last range at the end. A booking inside another, like [4, 6] inside [3, 20], is absorbed because `e` only grows.

**Complexity:** O(n log n) for the sort, O(n) for the sweep.
