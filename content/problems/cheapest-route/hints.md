Breadth-first search finds the route with the fewest roads, but tolls differ, so fewer roads is not always cheaper. Which town's cheapest cost can you be sure of first?
---
All tolls are positive. Among the towns you can reach, the one with the smallest known cost cannot get cheaper later: any other way to it passes through a town that already costs at least as much.
---
Dijkstra's algorithm: keep `dist` (∞ except `dist[start] = 0`) and a min-heap of `(cost, town)`. Pop the cheapest; skip it if it is out of date; otherwise relax each road from it, pushing `(dist[t] + toll, other)` when that improves the other town. Stop when `goal` is popped.
