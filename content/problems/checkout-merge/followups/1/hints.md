Trying every possible `lo` and looking up the nearest ticket in every line is
`customers × lines` work, up to 10⁹. Think of the base problem's merge: it visits all
tickets in increasing order, one front of a line at a time.
---
At any moment of the merge, the fronts of the lines (one per line) form a valid range:
from the smallest front to the largest. Which front can you advance without losing a
chance at a narrower range?
---
Keep the fronts in a min-heap and track the largest front. The range is `[heap minimum,
largest]`. Record it when it is strictly narrower than the best so far, then advance the
line that holds the minimum (push its next node, updating the largest). Stop when that
line runs out: from then on no range covers it.
