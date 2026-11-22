## Idea

The same sliding window as in the base problem, with a different rule to restore. Move `end` across the plays, counting each song inside `[start, end]` in a hash map, and delete a song from the map when its count drops to 0, so that the map's size is the number of different songs. When adding `plays[end]` pushes the size past `k`, advance `start`, removing plays, until it is back to `k`. The window is then the longest valid stretch ending at `end`; keep the largest. When `k` is at least the number of different songs in the log, the whole log is the answer.

## Complexity

O(n) time, since each play enters and leaves the window once; O(min(n, k)) extra memory for the map.
