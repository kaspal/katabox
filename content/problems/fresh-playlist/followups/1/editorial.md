## Idea

The base problem allowed each song once; here twice, and the same sliding window still works because shrinking a valid stretch keeps it valid. Move `end` across the plays and count each song inside `[start, end]` in a hash map. Only the song just added can have reached 3 plays, so while its count is above 2, drop `plays[start]` and advance `start`. The window is then the longest valid stretch ending at `end`, and the answer is the largest such length. The same code works for any limit on repeats.

## Complexity

O(n) time, since each position enters and leaves the window once. O(n) extra memory for the counts in the worst case.
