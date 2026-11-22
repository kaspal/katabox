A cell on its own needs 4 sides of fence. What happens to the total when two frozen cells of a pond sit side by side?
---
Explore each pond as in the base problem, with a stack or a queue from each unvisited `#`. While you visit a cell, look at its four neighbours.
---
For each of the four sides, add 1 to the pond's fence when the neighbour is off the map or is `.`; otherwise visit the neighbour if it is new. Collect one total per pond and sort the totals in decreasing order.
