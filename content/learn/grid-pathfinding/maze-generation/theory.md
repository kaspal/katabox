## The idea

The searches in this subject need mazes to run on, and a good maze can be grown with a depth-first search. Take a square grid of side `2k + 1`, all walls. The cells with two odd coordinates are **rooms**, `k × k` of them; the cells between two neighbouring rooms are the walls that may be knocked down. The sample opens the room at row 1, column 1 and puts it on a **stack**.

Each round looks at the room on top of the stack and lists the directions (up, right, down, left) whose room two cells away is inside the grid and still a wall, meaning not visited yet. If there are any, it picks one at random, opens the wall in between and that room, and pushes the room. If there are none, the room is a dead end for now and is popped: the search **backtracks** to the previous room. When the stack is empty, every room has been visited.

Each opened room hangs off exactly one earlier room, so the openings form a **spanning tree** of the `k²` rooms: `k² − 1` walls are removed, there are no loops, and exactly one path joins any two rooms. The output is the grid row by row, `1` for wall and `0` for open.

The random choices come from a tiny generator: `state = state × 48271 mod (2³¹ − 1)`, starting from the seed, and the pick is `state mod (number of options)`. The same `k` and seed always give the same maze, so a maze can be shared or replayed by two numbers.

### The default input

`k = 3`, seed 7: a 7 × 7 grid with nine rooms. The first number is 7 × 48271 = 337897; from the first room only right and down are possible, and 337897 is odd, so it picks the second: down. The finished maze is:

```
#######
#.#...#
#.###.#
#...#.#
###.#.#
#.....#
#######
```

Eight walls between rooms are gone, so 17 cells are open. The top-left room reaches the bottom-right one in eight steps (down, right, down, right through the rooms), but reaching the top-right room means winding through all three rows: depth-first mazes have long corridors.

## Steps

1. Fill a `(2k + 1) × (2k + 1)` grid with walls, open the room at (1, 1) and push it.
2. Look at the top room. List each direction whose room two steps away is inside and still a wall.
3. If the list is not empty, pick `options[rand() mod count]`, open the wall between and that room, and push it.
4. Otherwise pop the top room.
5. When the stack is empty, output the grid row by row.

## Cost

| Case | Cost |
|---|---|
| Whole run | `O(k²)`: each room is pushed once and popped once, checking four directions each round |
| Extra space | `O(k²)`: the grid, and a stack that can hold every room |

## Pitfalls

- **A zero state.** If the state is ever 0 it stays 0 and every pick is the first option; seeds start at 1.
- **Overflow.** The product `state × 48271` needs more than 32 bits; JavaScript numbers hold it exactly (it stays below 2⁴⁷), but in a language with 32-bit integers use 64-bit ones.
- **Recursion depth.** A recursive version can go `k²` calls deep; the samples use an explicit stack.

## When to use it

Use it to build levels for games and puzzles, and test maps for pathfinding: a perfect maze has exactly one route, so every search must find the same one. Other generators give mazes with a different feel: randomized Prim's grows the maze from a frontier of walls and makes many short dead ends, and randomized Kruskal's removes walls in random order, joining rooms that are not yet connected.

## Related topics

- [Depth-first search](learn:graphs/dfs): the walk that carves the maze.
- [Stack](learn:linear/stack): the structure that remembers where to backtrack.
- [Prim's minimum spanning tree](learn:graphs/prim): its randomized form is another maze generator.
- [Kruskal's minimum spanning tree](learn:graphs/kruskal): with union-find, another way to grow a spanning tree of rooms.
- [Breadth-first search on a grid](learn:grid-pathfinding/grid-bfs): solve the maze you just made.
