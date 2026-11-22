Look for a spot that has a slow direct road from the start but a faster route through another spot. Example 1 is exactly that shape.
---
Building the road lists and the heap itself are fine. Focus on the moment a spot's distance is written into `dist`, and on whether that value can ever be improved afterwards.
---
The code fixes a spot's distance the first time it sees it, from whatever road it happened to arrive by. A shorter route found later is ignored because `dist[u]` is no longer `-1`. In Dijkstra's algorithm a distance is final only when the spot is popped from the heap: skip popped spots that already have a distance, otherwise set `dist[v] = d`, and push neighbours without writing their `dist`.
