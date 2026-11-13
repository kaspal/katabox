## Approach: a sliding window with last positions

Walk `end` from left to right, keeping `start` so that `plays[start..end]` has no repeats, and `last[x]`, the latest position where song `x` was played.

For each new play `x = plays[end]`:

- if `x` was played at `last[x] ≥ start`, that earlier play is inside the window, so the window can only stay fresh by starting after it: `start = last[x] + 1`;
- record `last[x] = end` and update the best length `end − start + 1`.

`start` only moves forward, so each play enters and leaves the window once.

### Walkthrough

`[4, 8, 4, 2, 9, 8, 1]`: windows `[4]`, `[4, 8]`; the second 4 moves start to 1: `[8, 4]`; then `[8, 4, 2]`, `[8, 4, 2, 9]`; the second 8 (last seen at 1) moves start to 2: `[4, 2, 9, 8]`; then `[4, 2, 9, 8, 1]`, length 5.

**Complexity:** O(n) time, O(n) space. Growing a stretch from every start until a repeat is O(n · answer), O(n²) when the stretches are long.
