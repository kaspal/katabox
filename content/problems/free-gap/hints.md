Turn every `hh:mm` into minutes since midnight (`hh · 60 + mm`) and work with plain numbers; turn the answer back into `hh:mm` only when you print it.
---
Sort the day's slots by start time. Walking through them in that order, keep `free_from`: the first minute not covered by any slot seen so far (it starts at `open`). A slot that starts after `free_from` leaves a free stretch just before it.
---
For each slot `(s, e)` in start order: if `s − free_from` is longer than the best so far, remember the stretch `free_from..s`; then set `free_from = max(free_from, e)`. After the last slot, check the stretch `free_from..close` too. Replace the best only when a stretch is **strictly** longer, so the earliest wins ties. A best length of 0 means `BOOKED`.
