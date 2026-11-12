Every hop costs the same, so the question is about the number of links on the route, not their length. Which stations can be reached in one hop? In two?
---
Explore in rings: first station 0, then all its neighbours (1 hop), then all their unvisited neighbours (2 hops), and so on. The first ring that contains station `n − 1` gives the answer.
---
That is breadth-first search: a queue of stations, a `dist` array set when a station is first reached (`-1` = not yet). Each station enters the queue once, so it runs in O(n + links).
