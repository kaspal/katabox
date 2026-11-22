With one row the plan is always right. Try two rows and a single booking: does the other row stay free?
---
The loop that marks bookings touches exactly one cell. Look at how the hall is created before the loop: how many separate row lists really exist?
---
`[["."] * cols] * rows` (and `fill` with one array in TypeScript) puts the same row list in every position, so marking one row marks them all. Build a new list for each row: `[["."] * cols for _ in range(rows)]`.
