## Idea

The base problem needed one distance; a breadth-first search computes all of them at the
same price. Build an adjacency list with both directions of every link, start with
`hops = [-1] * n` and `hops[0] = 0`, and process a queue from station `0`: for each
neighbour `v` of the current station `u` with `hops[v] = -1`, set
`hops[v] = hops[u] + 1` and enqueue `v`. The queue holds stations in order of distance,
so the first time a station is seen is through a shortest route. Do not stop when
station `n − 1` is reached; run until the queue is empty. Stations in other components
keep `-1`.

## Complexity

O(n + links.length) time and memory.
