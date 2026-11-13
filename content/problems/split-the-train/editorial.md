## Approach: count, then cut

1. Count the wagons: n.
2. Part `i` (from 0) has `n // k` wagons, plus one if `i < n mod k`. This makes the lengths differ by at most one, with the longer parts first.
3. Walk the list again. For each part, remember its first node, move to its last node, cut the `next` link there, and continue from the next node. Parts of length 0 are empty (`null`, written `[]`).

### Walkthrough

`train = [1..7]`, `k = 3`: n = 7, 7 // 3 = 2 and 7 mod 3 = 1, so the sizes are 3, 2, 2. Cut after 3 and after 5.

**Complexity:** O(n + k) time, O(k) for the result. The nodes are reused.
