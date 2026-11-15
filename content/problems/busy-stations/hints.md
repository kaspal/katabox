Each trip mentions up to two stations, in two different columns. Can you turn the table into one list of (trip, station) pairs, so that a plain `GROUP BY station` does the counting?
---
Stack the two columns: `SELECT id, start_station AS station FROM trips` on top of `SELECT id, end_station FROM trips`. A round trip then gives the same pair `(id, station)` twice. Which set operator keeps it only once?
---
`UNION` (not `UNION ALL`) removes duplicate rows, so a round trip's two identical pairs collapse into one, while two different trips stay apart thanks to their `id`. Then `GROUP BY station HAVING COUNT(*) >= 3 ORDER BY trips DESC, station`, with `COUNT(*) AS trips`.
