Think in rounds: first the piers one crossing from Central, then the piers one crossing from those, and so on. The network can loop back on itself, so what makes the rounds stop?
---
`WITH RECURSIVE reach(pier, hops)` starts from `SELECT 'Central', 0`. The recursive part follows one more route: `JOIN routes r ON r.from_pier = reach.pier`, producing `r.to_pier, reach.hops + 1`. A condition `WHERE reach.hops < 3` only extends rows that still have a crossing left, which also ends the walk around any loop.
---
A pier can be reached by several paths of different lengths, so finish with `SELECT pier, MIN(hops) AS hops FROM reach WHERE pier <> 'Central' GROUP BY pier ORDER BY hops, pier`. Joining the two parts with `UNION` instead of `UNION ALL` drops repeated `(pier, hops)` rows and keeps the work small.
