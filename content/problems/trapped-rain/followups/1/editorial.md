## Idea

The edge columns form the starting "wall" around the unknown inside. Keep that wall in a min-heap keyed by water level. The lowest wall entry, at level `L`, settles its unvisited neighbours: every way out of the inside crosses the wall at level ≥ `L`, and this neighbour can leave through the entry at level `L`, so its water level is `max(L, h)` for its height `h`. It holds `max(0, L − h)` water and joins the wall at level `max(L, h)`. Repeating from the lowest wall point (Dijkstra's algorithm with "tallest column on the path" instead of "path length") visits every column once and computes exactly the definition. The base problem's two pointers are the one-row version of the same idea: always move the lower side.

## Complexity

O(R·C·log(R·C)) time for the heap operations and O(R·C) extra memory. The total fits easily in 32 bits: at most 4·10⁴ columns × 10⁴ = 4·10⁸.
