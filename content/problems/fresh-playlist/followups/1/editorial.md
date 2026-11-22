## Idea

One pass of the base problem's window gives, for every play `r`, `start[r]`: the first play
of the longest fresh stretch ending at `r` (move the left end past the previous play of the
same song). Any fresh stretch ending at `r` starts at `start[r]` or later, and
`start[r]` never decreases as `r` grows.

Inside a request `[a, b]`, the best stretch ending at `r` starts at `max(start[r], a)`:

- for the plays with `start[r] < a` (all of them come first, since `start` never
  decreases), that is `a..r`, and the longest is at the last of them;
- for the others, it is the whole stretch, length `len[r] = r − start[r] + 1`.

Binary search the first `r0` in `[a, b + 1)` with `start[r0] ≥ a`. The answer is the larger
of `r0 − a` and the maximum of `len` over `[r0, b]` (when `r0 ≤ b`). Build a sparse table
over `len` once, so each range maximum takes two lookups.

## Complexity

O(n log n) time and memory for the sparse table, then O(log n) per request. Rescanning
each request is O(n) per request, up to 10¹⁰ steps in all.
