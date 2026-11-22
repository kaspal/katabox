## The bug

The starter uses a heap, but it records distances too early:

```python
dist[start] = 0
heap = [(0, start)]
while heap:
    d, v = heapq.heappop(heap)
    for u, w in adj[v]:
        if dist[u] == -1:
            dist[u] = d + w
            heapq.heappush(heap, (d + w, u))
```

A spot gets its distance the first time any neighbour reaches it. That first road is not necessarily part of the shortest route: a cheaper path through a spot that is popped later is never considered, because by then `dist[u]` is no longer `-1`. The heap order is right, but the answer is written before the heap has had a chance to decide.

With `n = 3`, `roads = [[0, 1, 5], [0, 2, 1], [2, 1, 1]]` and `start = 0`: popping spot 0 sets `dist[1] = 5` and `dist[2] = 1`. Popping spot 2 finds spot 1 already set, so the code returns `[0, 5, 1]`. The route 0 → 2 → 1 takes 2 minutes, so the answer is `[0, 2, 1]`.

## The fix

A distance is final only when its spot leaves the heap, since nothing shorter can still be waiting. Drop the `dist[start] = 0` line and change the loop to:

```python
while heap:
    d, v = heapq.heappop(heap)
    if dist[v] != -1:
        continue
    dist[v] = d
    for u, w in adj[v]:
        if dist[u] == -1:
            heapq.heappush(heap, (d + w, u))
```

The TypeScript version needs the same change: remove `dist[start] = 0`, and after `heap.pop()` skip spots whose `dist` is already set, assign `dist[v] = d`, then push neighbours without touching their `dist`.

**Complexity:** O((n + m) log m) time for m roads, O(n + m) space.
