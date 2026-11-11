## Approach: breadth-first search

If the start or the goal is a shelf, return -1. Otherwise run a BFS from the top-left cell:

1. Put the start in a queue with distance 0 and mark it visited.
2. Pop the next cell. If it is the goal, return its distance.
3. For each of its four neighbours that is inside the grid, free and not visited, mark it visited and push it with distance + 1.
4. If the queue empties, the goal is unreachable: return -1.

BFS pops cells in order of distance, so the first time the goal is popped its distance is the shortest. Marking cells when they are pushed (not when popped) keeps each cell in the queue at most once.

### Walkthrough

```
0 0 0
1 1 0
0 0 0
```

| distance | cells reached |
| --- | --- |
| 0 | (0,0) |
| 1 | (0,1) |
| 2 | (0,2) |
| 3 | (1,2) |
| 4 | (2,2): the goal |

**Complexity:** O(rows·cols) time and space: every cell is pushed at most once and has four neighbours.
