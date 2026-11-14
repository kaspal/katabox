## The idea

Given a grid of letters and a word, can the word be spelled by a **path** that moves between neighbouring cells (up, right, down or left, never diagonally) and never uses a cell twice? Backtracking grows the path one letter at a time and steps back whenever the next letter cannot be reached.

The sample tries every cell as the start, in reading order. `walk(r, c, k)` asks whether the path can go on from cell `(r, c)` as letter `k` of the word. It fails at once if the cell's letter is not `word[k]` or the cell is already on the path. Otherwise the cell is **stepped onto** (pushed on the path). If it was the last letter, the word is found. If not, the four neighbours are tried in the order up, right, down, left, each as letter `k + 1`. When none of them leads to the end, the cell is **stepped back** off the path (popped) and the call fails, so the caller tries its next direction. The first complete path wins.

The output is `[1, then the path's (row, column) pairs]` when the word is found, or `[0]` when it is not. The input is typed as a letter map, rows separated by `/`, and the word, letters a to z. The board highlights the current path.

### The default input

The map `wqzk/arym/tels/bnow` and the word `water`:

```
w q z k
a r y m
t e l s
b n o w
```

The start `(0, 0)` holds `w`. Up is outside, right is `q`, down is `a`: step. From `(1, 0)`, up is the `w` (wrong letter), right is `r`, down is `t`: step. From `(2, 0)`, up is the `a`, right is `e`: step. From `(2, 1)`, up is `r`, the last letter. The word bends down, right and up again without a single step back, and the second `w` at `(3, 3)` is never tried.

The output is `[1, 0, 0, 1, 0, 2, 0, 2, 1, 1, 1]`. The "False starts" preset shows the other side: several promising starts, all ending in dead ends.

## Steps

1. For each cell in reading order, call `walk(r, c, 0)`; stop at the first success.
2. In `walk(r, c, k)`: fail if the letter differs from `word[k]` or the cell is on the path.
3. Push the cell. If `k` is the last position, succeed.
4. Try the neighbours up, right, down, left with `k + 1`; succeed if any does.
5. Otherwise pop the cell and fail.

## Cost

| Case | Cost |
|---|---|
| Best | `O(R · C)`: the scan over the starts when the first letter rarely matches |
| Average | `O(R · C · 3ᴸ)`: after the first step, at most 3 new directions per letter |
| Worst | `O(R · C · 3ᴸ)`, for grids full of repeated letters |
| Extra space | `O(L)` for the path and the call depth |

## Pitfalls

- **Forgetting to step back.** A cell left on the path blocks every later branch that would need it.
- **Reusing a cell.** Without the path check, a word like `aba` would be found in a grid that only has `ab`.
- **Slow path check.** The sample scans the path for each cell; a grid of "on path" flags makes the check constant time.

## When to use it

Use it to trace one word, route or shape through a grid. With **many** words, searching for each one separately repeats the same work. Instead, store all the words in a [trie](learn:trees/trie) and walk the grid once from each start, following the trie's children: a branch stops as soon as no word begins with the letters so far. The linked problem, Word Grid Hunt, needs that idea.

## Related topics

- [Trie](learn:trees/trie): many words, one walk.
- [Depth-first search](learn:graphs/dfs): the same go-deep-then-back walk on a graph.
- [Sudoku](learn:backtracking/sudoku): backtracking over the cells of a grid.
- [Breadth-first search on a grid](learn:grid-pathfinding/grid-bfs): the shortest route through a grid instead of an exact spelling.
