## Approach: Dijkstra on the hardest step

Define `effort(trail)` as its hardest step. We want the trail to the bottom-right spot with the smallest effort. Dijkstra's algorithm works for any "length" that never decreases as the trail grows, and `max` qualifies: extending a trail by a step of difficulty `s` gives `max(effort, s) ≥ effort`.

```
best[start] = 0; heap = [(0, start)]
while heap:
    d, spot = pop smallest
    if spot is the end: return d
    if d > best[spot]: continue          # an outdated heap entry
    for each neighbour nb:
        nd = max(d, |height[nb] - height[spot]|)
        if nd < best[nb]: best[nb] = nd; push (nd, nb)
```

When a spot leaves the heap, no trail can reach it with smaller effort, because every other entry in the heap is at least as large and steps never lower the effort.

### Walkthrough

In Example 1 the heap expands 10 (0), then 12 (2), 15 (3), 17 (3). From 17, the end 40 is offered at 23 and 50 at 33. From 10 the spot 30 was offered at 20, so 30 comes out next (20), then 31 (20) and 33 (20). From 33 the end is offered again at `max(20, 7) = 20`, which beats 23, and the end leaves the heap with 20.

### Other ways

- **Binary search on the limit** plus a BFS for each try: O(r·c·log(max height)).
- **Union-find:** sort all neighbour pairs by difference and join them in order until start and end are connected; the last difference used is the answer.

**Complexity:** O(r·c·log(r·c)) time and O(r·c) space. Trying every distinct step difficulty in increasing order, with a fresh search each time, is O(r·c) per try and up to about 2·r·c tries.
