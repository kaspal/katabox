## Idea

Put the first node of every line in a min-heap keyed by `(ticket, line index)` and remember
`largest`, the biggest ticket among the fronts. The fronts hold one ticket from each line,
so `[minimum, largest]` is a valid range.

Repeat: compare that range with the best so far and keep it only if it is **strictly**
narrower. Then pop the minimum node and push its `next`, updating `largest`. When the
popped line has no next node, stop.

Why it is enough: any narrowest range `[lo, hi]` has, in every line, a ticket inside it.
While the heap minimum is below `lo`, advancing the minimum line cannot skip that line's
ticket inside `[lo, hi]`, because it is at least `lo`. So the scan reaches a moment where
every front is in `[lo, hi]`, and records a range at least as narrow. The minimum only
grows during the scan, so the first range of the best width also has the smallest `lo`,
which is why only strictly narrower ranges replace the best.

## Complexity

O(N log k) time for `N` customers in `k` lines and O(k) extra memory for the heap.
