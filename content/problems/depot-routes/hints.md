Every call is answered from the same junction, the base. What if you worked out the answer for every junction before looking at the calls?
---
One run of Dijkstra's algorithm from junction 1 finds the shortest time to every junction at once, in O((N + M) log N) with a binary heap. Running a fresh search for each call multiplies that cost by `Q`.
---
Build adjacency lists (each street goes into both junctions' lists), then run Dijkstra once from junction 1 with a min-heap of `(time, junction)` pairs, skipping popped pairs whose time is larger than the junction's best. Keep "infinity" for junctions never reached; each call is then a lookup of `dist[t]`, printing `-1` for infinity.
