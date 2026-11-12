## Approach: breadth-first search

With every hop costing 1, the fewest hops to each station are found by exploring the network in rings around station 0: breadth-first search (BFS).

```
dist = [-1] * n; dist[0] = 0
queue = [0]
for v in queue:                    # stations come out in order of distance
    for u in neighbours[v]:
        if dist[u] == -1:
            dist[u] = dist[v] + 1
            queue.append(u)
return dist[n − 1]
```

A station is first reached through a shortest route, because the queue hands out stations in non-decreasing distance: all stations at distance `d` are processed before any at `d + 1`. A station that is never reached keeps `-1`, which is exactly the answer for "no route". Duplicate links only add a repeated neighbour, which the `dist` check skips.

### Walkthrough

Example 1: from 0 the queue gets 1 and 3 (distance 1); from 1 it gets 2, from 3 it gets 4 (distance 2). Station 4 is reached with 2 hops.

**Complexity:** O(n + m) time and memory for `m` links. Relaxing every link again and again until nothing improves (as Bellman–Ford does) is O(n · m).
