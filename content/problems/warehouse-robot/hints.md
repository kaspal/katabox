Think of each free cell as a node, connected to its free neighbours. You need the fewest edges from the start node to the goal node.
---
When every move costs the same, exploring cells in order of distance finds the shortest route. Which traversal does that?
---
Breadth-first search: a queue starting at the top-left cell with distance 0. Pop a cell, push each free, unvisited neighbour with distance + 1, and mark it visited when you push it. Stop when you pop the goal.
