## Approach: dynamic programming over subsets (Held–Karp)

The order of the stops already visited does not matter for the rest of the round: only **which** stops have been visited and **where** the courier stands now. That gives the state `(mask, last)`, where bit `j` of `mask` says whether stop `j` has been visited (bit 0, the depot, is always set) and `last` is the current stop.

`best(mask, last)` is the fewest minutes to visit every stop not in `mask` and return to the depot:

```
best(full, last) = dist[last][0]
best(mask, last) = min over j not in mask of dist[last][j] + best(mask | (1 << j), j)
answer = best(1, 0)
```

Memoising `best` (or filling a table over masks in increasing order) evaluates each of the 2ⁿ · n states once, each in O(n).

**Why it is correct.** Any round is the depot, then a first stop `j`, then a round of the remaining stops ending at the depot. Given `(mask, last)`, the cheapest completion is the cheapest choice of next stop plus the cheapest completion after it, which is exactly the recurrence. Rides always go straight, so no other paths need to be considered, even when the distances do not obey the triangle inequality.

### Walkthrough

Example 1: `best({0,1,2}, 1) = dist[1][0] = 1` and `best({0,1,2}, 2) = 15`. Then `best({0,2}, 2) = dist[2][1] + 1 = 8`, `best({0,1}, 1) = dist[1][2] + 15 = 21`. Finally `best({0}, 0) = min(2 + 21, 9 + 8) = 17`.

**Complexity:** O(2ⁿ · n²) time and O(2ⁿ · n) memory: about 6·10⁵ steps for n = 12. Trying every order is O(n!).
