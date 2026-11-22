In the wall, the lower of the two sides set the water level. In a grid, water escapes through the **lowest** point of whatever surrounds it. Where is the lowest point of the outer ring?
---
Look at the lowest column on the edge. Its unvisited neighbour cannot hold water above that height, or the water would spill right over it. Can you grow the known area inward, always from its lowest boundary point?
---
Put every edge column in a min-heap keyed by its level (its height). Pop the lowest; for each unvisited neighbour, add `max(0, level − height)` water and push it with level `max(level, height)`. Each column is visited once.
