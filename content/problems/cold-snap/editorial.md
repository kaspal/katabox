## Approach: a segment tree with lazy shifts

Build a segment tree over the stations: node `v` covers a segment `[lo, hi]` and stores `mn[v]`, the lowest forecast in it.

- **Shift [a, b] by delta:** if the node's segment lies inside `[a, b]`, add `delta` to `mn[v]` and to `lazy[v]` (a shift owed to the children) and stop. If it only overlaps, first **push** `lazy[v]` down to both children, recurse into them, then set `mn[v] = min(mn[left], mn[right])`.
- **Coldest [a, b]:** if the segment lies inside, return `mn[v]`; if it is disjoint, return +∞; otherwise push and return the smaller of the two children's answers.

A range is covered by O(log n) maximal nodes, so each operation visits O(log n) nodes.

### Walkthrough

`temps = [5, −2, 7, 0, 3]`, shift [1, 3] by +4: the nodes covering exactly 1..3 get +4 in `mn` and `lazy`; their ancestors recompute their minimum. `coldest [1, 3]` then reads those nodes: min(2, 11, 4) = 2.

**Complexity:** O((n + q) log n) time, O(n) space. Applying each shift station by station and scanning each range is O(n·q).
