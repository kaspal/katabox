## Approach: a stack of days still waiting

Go through the days left to right with a stack of the indices that have not found a warmer day yet. Their temperatures never increase from the bottom of the stack to the top: a day that was warmer than the top would already have popped it.

For day `i`:
1. While the top `j` has `temps[j] < temps[i]`, pop it and set `wait[j] = i − j`.
2. Push `i`.

Days left on the stack at the end keep `wait = 0`.

Every index is pushed once and popped at most once, so the total work is linear.

### Walkthrough

`temps = [3, 1, 4, 1, 5, 2]`:

| day | temp | popped (wait) | stack after |
| --- | --- | --- | --- |
| 0 | 3 | | [0] |
| 1 | 1 | | [0, 1] |
| 2 | 4 | 1 (1), 0 (2) | [2] |
| 3 | 1 | | [2, 3] |
| 4 | 5 | 3 (1), 2 (2) | [4] |
| 5 | 2 | | [4, 5] |

**Complexity:** O(n) time and O(n) space for the stack. Scanning forward from each day is O(n²) when the weather stays flat or cold for a long time.
