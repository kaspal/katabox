Scanning forward from every day works, but a long cold spell makes every day scan almost to the end: O(n²).
---
Walk through the days in order and keep the days that are still waiting for a warmer one. When a warm day arrives, which waiting days does it answer?
---
Keep a stack of indices whose temperatures never increase from bottom to top. For each day `i`, pop every index `j` with `temps[j] < temps[i]` and set `wait[j] = i - j`; then push `i`. Whatever stays on the stack waits forever (0).
