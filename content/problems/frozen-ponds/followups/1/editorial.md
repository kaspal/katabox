## Idea

Flood each pond exactly as in the base problem, but count fence sides instead of cells. When visiting a cell, check its four neighbours: one off the map or holding `.` adds a side of fence; a `#` neighbour adds nothing (it belongs to the same pond) and is pushed if not yet seen. Every side of every frozen cell is checked exactly once, so each pond's total is its fence length. Equivalently, the fence is 4 · cells − 2 · (pairs of its cells that touch). Sort the totals in decreasing order.

## Complexity

O(R·C) time for the flood fill, plus O(P log P) to sort the P ponds; O(R·C) extra memory for the visited marks and the stack.
