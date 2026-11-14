## The idea

Dijkstra spreads out evenly in every direction, even away from the goal. **A\*** gives it a sense of direction. For each cell it adds two numbers: `g`, the cheapest cost found so far from S, and `h`, an estimate of what is left. Here `h` is the **Manhattan distance** to G: rows apart plus columns apart. The queue is ordered by `f = g + h`, then by `h` (on a tie, prefer the cell nearer G), then by row and column.

The estimate never **overestimates**: every step costs at least 1 and moves at most one row or column closer, so reaching G from a cell costs at least `h`. It is also **consistent**: one step changes `h` by at most 1 and costs at least 1, so `f` never drops along a route. Together they guarantee that when G leaves the queue, its `g` is the cheapest cost, exactly what Dijkstra returns. The rest is Dijkstra: relax neighbours (up, right, down, left) through `g`, push `(g + h, h, row, column)`, and skip stale entries whose `g` (`f - h`) is above the cell's best. No path means `[-1]`.

### The default input

The map is BFS's maze, `S.#....../.##.###.#/....#...#/##.##.#.#/...#..#.G`, with every open cell costing 1. S is 4 rows and 8 columns from G, so `h = 12` and `f = 12`. A step towards G adds 1 to `g` and takes 1 from `h`, so `f` stays 12; a step away raises it by 2. A\* takes out nine cells at `f = 12`: S, the dead end to its right, the left edge down to row 2, row 2 as far as the wall, and the gap in column 2 down to the bottom row. Then the walls force a choice at `f = 14`: the bottom-row cell left of that gap (smaller `h`) and the cell above the end of row 2's corridor. The route must climb two rows, away from G, so every remaining cell has `f = 16`.

A\* takes 23 cells out of the queue; Dijkstra on the same map takes 26. The output is `[16]`, the same as BFS and Dijkstra. On the swamp preset the gap is wider: 20 entries out (one stale) against Dijkstra's 31.

## Steps

1. Set the start's `g` to 0 and push `(h, h, start row, start column)`.
2. Pop the smallest entry `(f, h, r, c)`. If `f - h` is larger than the cell's `g`, skip it.
3. If the cell is G, return its `g`.
4. For each neighbour that is inside the grid and not a wall: if its `g` is unknown or `g + entry cost` beats it, record it and push `(new g + h, h, row, column)`.
5. If the queue empties first, return `-1`.

## Cost

| Case | Cost |
|---|---|
| Best | `O(path length)` with a perfect `h`: only cells on the route leave the queue |
| Typical | fewer cells than Dijkstra |
| Worst | `O(R·C · log(R·C))`: `h` does not help, for example when G is walled off |
| Extra space | `O(R·C)` for `g` and the queue |

## Pitfalls

- **An estimate that overestimates.** Doubling the Manhattan distance, say, can make A\* return a route that is not the cheapest.
- **Stopping when G is pushed.** As in Dijkstra, only popping G makes its cost final.
- **Diagonal moves with Manhattan distance.** If diagonals were allowed, Manhattan could overestimate; the samples move in four directions only.

## When to use it

Use A\* for one start and one goal when you can estimate the remaining cost cheaply: game characters, robots and route planners. Race it against Dijkstra in compare mode: both report the same cost, but A\* usually settles fewer cells.

## Related topics

- [Dijkstra on a weighted grid](learn:grid-pathfinding/grid-dijkstra): A\* with `h = 0`.
- [Greedy best-first search](learn:grid-pathfinding/greedy-best-first): `h` alone, faster but not always cheapest.
- [Breadth-first search on a grid](learn:grid-pathfinding/grid-bfs): fewest steps when every step costs 1.
- [Binary heap and priority queue](learn:trees/binary-heap): the queue behind the search.
