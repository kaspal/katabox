## The idea

A sudoku grid of size `s` (4 or 9) is split into boxes of `√s × √s`. Fill every empty cell with a digit from 1 to `s` so that no row, column or box holds a digit twice. Backtracking treats each empty cell as one choice and guesses its way through them, undoing a guess as soon as it leads nowhere.

The sample's `fill()` looks for the **first empty cell in reading order** (row by row, left to right). If there is none, the grid is solved. Otherwise it tries the digits **in ascending order**; a digit is allowed when `ok` finds it nowhere in the cell's row, column or box. It writes the digit and calls `fill()` again. If that call reports success, the success is passed straight up. If not, the cell is reset to empty (**undo**) and the next digit is tried. When no digit fits, `fill()` reports failure, and the cell before it gets its next digit. The output is the solved grid row by row, or `[-1]` when the givens have no solution. The grid is typed one row at a time, rows separated by `/`, with `.` for an empty cell; there is also a 9×9 preset. Givens are shown in bold.

### The default input

The 4×4 grid `1.../..2./.3../...4`. Row 0: cell (0, 1) gets 2, and cell (0, 2) takes 3, the first digit that fits. Then cell (0, 3) needs 4, but column 3 already has a 4: dead end. The 3 is undone, (0, 2) becomes 4 and (0, 3) becomes 3. Row 1 fills as 3, 4, 1 around its given 2. In row 2, (2, 0) first gets 2 and (2, 2) gets 1, but (2, 3) would need 4 again: dead end. No other digit fits (2, 2), so the search goes back to (2, 0), which becomes 4; now (2, 2) is 1 and (2, 3) is 2. Row 3 ends as 2, 1, 3.

In all, 15 digits are placed and 3 are undone. The output is `[1, 2, 4, 3, 3, 4, 2, 1, 4, 3, 1, 2, 2, 1, 3, 4]`.

## Steps

1. Find the first empty cell; if there is none, report success.
2. For each digit from 1 to `s`: if its row, column and box do not hold it, write it and fill the rest.
3. If the rest succeeds, report success; otherwise clear the cell and try the next digit.
4. If no digit works, report failure so the previous cell changes its guess.

## Cost

| Case | Cost |
|---|---|
| Best | Fast with few gaps: each one is filled after a few checks |
| Average | Exponential in the gaps: each empty cell can multiply the guesses |
| Worst | Exponential in the gaps, up to `s` guesses per empty cell |
| Extra space | `O(cells)`: the grid copy, and one call per empty cell |

## Pitfalls

- **Not clearing the cell on undo.** A stale digit keeps blocking its row, column and box, and the search for the first empty cell skips it.
- **Losing the success.** Once a deeper call has solved the grid, return at once; trying more digits would undo the solution.
- **The box corner.** The box starts at `r − r % b`, `c − c % b` for box side `b`, not at `r / b`.
- **Trusting the givens.** The sample never checks two givens against each other; a careful solver checks them before searching.

## When to use it

Use it for puzzles where cells must take values under row, column or group rules: sudoku, timetables with clashes, colouring a small map. Real solvers add two ideas. **Choose the most constrained cell**, the empty cell with the fewest allowed digits, instead of the first one: dead ends then show up much sooner. And **propagate constraints**: after each placement, remove that digit from its neighbours' options and fill any cell left with a single option, often solving easy grids without guessing at all.

## Related topics

- [N-Queens](learn:backtracking/n-queens): place, check and remove, one row at a time.
- [Word search](learn:backtracking/word-search): backtracking over cells of a grid.
- [Bit manipulation basics](learn:foundations/bit-manipulation): one bitmask per row, column and box makes the check instant.
- [Depth-first search](learn:graphs/dfs): the same go-deep-then-back order.
