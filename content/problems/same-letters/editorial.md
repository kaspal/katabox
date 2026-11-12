## Approach: count the tiles

Two bags are twins exactly when they have the same count of every letter.

1. If the lengths differ, return `false` straight away.
2. Keep an array of 26 counters. For each position, add 1 for `a[i]` and subtract 1 for `b[i]`.
3. The bags are twins if every counter is 0.

### Walkthrough

`a = "tool"`, `b = "lots"`: after the loop, `o` is at +1 (two in `a`, one in `b`) and `s` at −1, so the answer is `false`.

**Complexity:** O(n) time and O(1) extra space (26 counters). Sorting both strings and comparing them is also correct, in O(n log n).
