## The idea

A\* weighs two things: what a route has cost so far (`g`) and how far it still looks (`h`). **Greedy best-first search** drops the first one. It always takes out the cell that looks closest to G, by the **Manhattan distance** `h` alone, as if the cost of getting there did not matter. The queue holds `(h, row, column)` entries, smallest `h` first, ties broken by row and then column.

There are no distance updates. When a neighbour (up, right, down, left) is first discovered, it is marked by recording what was paid to reach it: the current cell's amount plus the neighbour's entry cost (1 for an open cell, 2 to 9 for a weighted one). A marked cell is never touched again, even if a cheaper way to it turns up later. When G leaves the queue, the answer is what was paid along the route that discovered it. If the queue runs dry, the answer is `[-1]`.

That makes greedy search fast when the way is clear, but blind to two things: walls between a cell and G, and costs. The route it reports is a valid one, not necessarily the cheapest.

### The default input

The "trap" map, `......../.#####../.....#../S....#.G/.....#../.#####../........`, has a box of walls that is open towards S. G lies straight ahead, so `h` falls from 7 to 3 as the search runs along row 3 into the box. There the wall blocks it, and every cell left inside looks worse, so the search fills the whole pocket on its way back out. Only then does it climb the left edge, run along the top row with `h` falling again, and come down to G.

It takes 28 cells out of the queue; BFS needs 43 on the same map. The output is `[13]`: 3 up, 7 across and 3 down. Here that happens to be the cheapest cost too, but nothing guarantees it. On the tiny map `S9G/...`, greedy steps into the 9 because it is right next to G, and pays 10; the cheapest route goes around for 4. The "A toll lane" input shows the same on a bigger map: greedy pays 14 where 8 would do.

## Steps

1. Mark the start as paid 0 and push `(h, start row, start column)`.
2. Pop the entry with the smallest `h`. If it is G, return what was paid to reach it.
3. For each neighbour that is inside the grid, not a wall and not yet marked: mark it with this cell's amount plus its entry cost, and push `(h, row, column)`.
4. If the queue empties first, return `-1`.

## Cost

| Case | Cost |
|---|---|
| Best | `O(path length)`: a clear line of sight to G |
| Typical | often few cells, but the path is not always the cheapest |
| Worst | `O(R·C · log(R·C))`: every cell is queued once |
| Extra space | `O(R·C)` for the marks and the queue |

## Pitfalls

- **Expecting the cheapest route.** Greedy search ignores what it has paid; use A\* or Dijkstra when cost matters.
- **Dead ends shaped like the trap.** A pocket facing the start pulls the search in, and it must empty it before trying anything else.
- **Reading the answer as a distance.** It is the cost of one route found, not a guaranteed minimum.

## When to use it

Use it when any route will do and speed matters more than cost: quick checks in open maps, or as a first guess. Race it against A\* in compare mode: on open maps it often explores fewer cells, and on weighted maps you can watch it pay more.

## Related topics

- [A\* search](learn:grid-pathfinding/a-star): adds the cost so far back in and gets the cheapest route.
- [Dijkstra on a weighted grid](learn:grid-pathfinding/grid-dijkstra): cost only, no estimate.
- [Breadth-first search on a grid](learn:grid-pathfinding/grid-bfs): steps only, no estimate.
- [Binary heap and priority queue](learn:trees/binary-heap): the queue behind the search.
