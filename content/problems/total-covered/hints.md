Try a long shift that contains a short one, such as `shifts = [[1, 10], [2, 3]]`. Which shift does the merge loop see first?
---
The merge step only works if, when a shift arrives, no later shift can start before the current block does. Check how the shifts are ordered before the loop.
---
The shifts are sorted by their end time, so a shift that starts earlier can arrive after the block has already fixed its start, and the time before that start is lost. Sort by start time instead: plain `sorted(shifts)` (in TypeScript, compare `x[0] - y[0]`).
