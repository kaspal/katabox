## The idea

A grid is a graph in disguise: every open cell is a node, and it is joined to the cells directly above, to the right, below and to the left of it. **Breadth-first search** on that graph spreads out from the start in rings, one step at a time, so the first time it reaches a cell it has used the fewest possible steps. That makes it the right tool for "how many moves to get from S to G?" when every move costs the same.

The sample keeps a grid of distances, all `-1` except the start's `0`, and a **queue**, first in, first out. It takes the cell at the front; if that cell is G, it answers with G's distance. Otherwise it tries the four neighbours, always in the order up, right, down, left. A neighbour inside the grid, not a wall and still at `-1` gets "my distance + 1" and joins the back of the queue. Setting the distance marks it as seen, so no cell is queued twice. If the queue runs dry first, G cannot be reached and the answer is `[-1]`.

BFS counts **steps**, not cost: a weighted cell (2 to 9) is just an open cell to it.

### The default input

The map is `S.#....../.##.###.#/....#...#/##.##.#.#/...#..#.G`: five rows, nine columns, S in the top-left corner and G in the bottom-right. Right after S, its right-hand neighbour leaves the queue (it was found before the one below), but walls close it in: a dead end at distance 1. The real route goes down the left edge to row 2, right to distance 5, up the column at index 3 to the top row (distance 7), along the top to the column at index 7 (distance 11), down that column to row 4 (distance 15) and one step right into G.

Along the way BFS also fills the dead ends: the bottom-left corner (distance 8), the top-right corner (12) and the short corridor that leaves column 7 to the left in row 2. It takes 25 cells out of the queue, G last, and the output is `[16]`.

## Steps

1. Set every distance to `-1`, the start's to `0`, and put the start in the queue.
2. Take the cell at the front. If it is G, return its distance.
3. For each neighbour (up, right, down, left): if it is inside the grid, not a wall and still `-1`, set its distance to this cell's plus 1 and add it to the back of the queue.
4. Repeat; if the queue empties without reaching G, return `-1`.

## Cost

| Case | Cost |
|---|---|
| Whole search | `O(R·C)`: each cell is queued at most once and checks four neighbours |
| Extra space | `O(R·C)` for the distances and the queue |

## Pitfalls

- **Marking cells when they leave the queue.** A cell can then be queued by up to four neighbours. Mark it the moment it is discovered, as the samples do.
- **Mixing up rows and columns.** Up and down change the row, left and right change the column; check `0 ≤ r < R` and `0 ≤ c < C` separately.
- **Trusting BFS with weights.** On Dijkstra's default map BFS answers 11 steps, but the cheapest route costs 27.

## When to use it

Use grid BFS for the fewest moves in mazes, game boards, robot floors and flood-style spreading where every step costs the same. In compare mode you can race it against another search on the same map: on a map without weights it ties Dijkstra on the answer and spends less effort per cell.

## Related topics

- [Breadth-first search](learn:graphs/bfs): the same algorithm on a general graph.
- [Queue and circular buffer](learn:linear/queue-ring-buffer): the queue that drives the search.
- [Dijkstra on a weighted grid](learn:grid-pathfinding/grid-dijkstra): when cells cost different amounts.
- [A* search](learn:grid-pathfinding/a-star): a search steered towards G.
- [0-1 BFS](learn:graphs/zero-one-bfs): BFS with a deque when steps cost 0 or 1.
