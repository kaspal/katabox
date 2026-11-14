## The idea

On some maps a step is not just a step: mud, sand or a toll booth makes certain cells dearer to enter. Here an open cell costs 1 to enter and a cell marked 2 to 9 costs that much, while 1 is still a wall. BFS would count every step as 1, so it can pick a short route straight through the expensive cells. **Dijkstra's algorithm** instead finds the route with the smallest **total cost**.

It keeps the best known cost for every cell and a **priority queue** of `(distance, row, column)` entries, smallest distance first, ties broken by the smaller row and then the smaller column. The cell taken out of the queue is settled: nothing found later can reach it more cheaply, because every remaining route only adds cost. If it is G, its distance is the answer. Otherwise each neighbour (up, right, down, left) that is not a wall is **relaxed**: if this cell's distance plus the neighbour's entry cost beats the neighbour's best so far, record it and push a new entry. Older, larger entries for the same cell stay in the queue; when one comes out it is **stale** and is skipped. No path means `[-1]`.

Python uses `heapq`. TypeScript has no built-in priority queue, so the TS sample keeps an array sorted by `(distance, row, column)` after every push and takes from the front: plenty for a small grid.

### The default input

The map is `S..99..../.#.99.##./.#.99..#./...99...G`: a swamp two columns wide splits the grid from top to bottom, so every route must cross it and pay 9 + 9 = 18. Dijkstra first settles the ten open cells left of the swamp, with distances 0 to 5. Entering the swamp from them gives its first column 11, 12, 13 and 14 from top to bottom; crossing the second column costs another 9 (20 at the top). From there the cheapest routes walk the remaining cost-1 cells to G.

All 31 open cells are settled, G last; no stale entry comes out on this map. The output is `[27]`: eleven cells entered, nine of them at 1 and two at 9. BFS on the same map answers 11, the step count.

## Steps

1. Set the start's distance to 0, every other to "unknown", and push `(0, start row, start column)`.
2. Pop the smallest entry `(d, r, c)`. If `d` is larger than the cell's best distance, skip it.
3. If the cell is G, return `d`.
4. For each neighbour that is inside the grid and not a wall: if it is unknown or `d` plus its entry cost is smaller than its distance, record the new distance and push the entry.
5. If the queue empties first, return `-1`.

## Cost

| Case | Cost |
|---|---|
| Whole run with a binary heap | `O(R·C · log(R·C))`: up to four pushes per cell, each `O(log(R·C))` |
| Extra space | `O(R·C)` for the distances and the queue |

The TS sorted array costs more per push; a real program would use a binary heap.

## Pitfalls

- **Adding the cost of the cell you leave.** The cost belongs to the cell you enter; the start's own cost is never paid.
- **Stopping when G is first reached by relaxing.** Only popping settles a cell; a cheaper route can still turn up before G leaves the queue.
- **Forgetting the stale check.** Processing an old entry again does no harm to the answer but wastes work.

## When to use it

Use it for game maps with terrain, route planning with tolls, and any grid where cells cost different non-negative amounts. In compare mode, race it against BFS on a weighted map to see different answers, or against A* to see how many fewer cells A* settles for the same cost.

## Related topics

- [Dijkstra's shortest paths](learn:graphs/dijkstra): the same algorithm on a general graph.
- [Binary heap and priority queue](learn:trees/binary-heap): the queue behind the algorithm.
- [Breadth-first search on a grid](learn:grid-pathfinding/grid-bfs): the case where every step costs 1.
- [A* search](learn:grid-pathfinding/a-star): Dijkstra steered towards G by an estimate.
