## Idea

Search a graph with two layers. State `(v, 0)` means "at station `v`, no link turned yet"
and `(v, 1)` means "at station `v`, the turn is spent". Every link `[a, b]` gives the
edges `(a, 0) → (b, 0)` and `(a, 1) → (b, 1)`, and also `(b, 0) → (a, 1)`: crossing it
backwards is the moment you turn it. A breadth-first search from `(0, 0)` gives the
fewest hops to every state; the answer is the smaller of the distances to `(n − 1, 0)`
and `(n − 1, 1)` that exist, or `-1` if neither was reached.

Could the search use a link forwards and also turned, which the rules forbid? A route
that crosses the same link in both directions visits a station twice, and a shortest
route never does, so the distances found are always achievable with one real turn.

## Complexity

O(n + links.length) time and memory: the layered graph has 2n states and 3 edges per
link.
