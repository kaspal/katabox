## Approach: a monotonic stack

Every poster has a lowest covered panel `k`, and for that `k` the widest poster runs from just after the nearest lower panel on the left to just before the nearest lower panel on the right. So the answer is the best of n candidates, one per panel.

Walk the panels left to right with a stack of positions whose heights increase from bottom to top. Add a sentinel panel of height 0 at the end. For each position `i`:

- while the panel on top of the stack is **higher** than `panels[i]`, pop it as `k`: panel `i` is the nearest lower panel on its right, and the panel now on top of the stack (or the left edge) is the nearest lower-or-equal on its left. Its poster has width `i − top − 1` (or `i` with an empty stack) and area `panels[k] · width`;
- push `i`.

Equal heights do not break this: of two equal panels, the later one pops the earlier one only when something lower arrives, and then the later one's poster covers both.

### Walkthrough

`[3, 1, 4, 4, 2, 5]`: 1 pops 3 (width 1, area 3); 2 pops both 4s (the second one: width 1, area 4; the first: width 2, area 8); the sentinel pops 5 (5), 2 (width 4, area 8) and 1 (width 6, area 6). The best is 8.

**Complexity:** O(n) time, O(n) space. Trying every left end with a running minimum is O(n²).
