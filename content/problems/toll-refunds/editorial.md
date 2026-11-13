## Approach: Bellman–Ford with a final check

1. `cost[start] = 0`, every other town ∞.
2. Repeat n − 1 times: for every road `[a, b, c]` with `cost[a]` finite, set `cost[b] = min(cost[b], cost[a] + c)`. Stop early if a round changes nothing.
3. One more round: if any road with a finite `cost[a]` could still lower `cost[b]`, a negative loop is reachable from `start`: return `[]`.
4. Otherwise return the costs, writing 10⁹ for towns still at ∞.

**Why n − 1 rounds:** without a reachable negative loop, a cheapest route never repeats a town, so it has at most n − 1 roads; after round k, every town whose cheapest route has k roads is correct.

**Why Dijkstra fails:** Example 1 would settle town 1 at cost 4 before discovering 0 → 2 → 1 at cost 2.

**Complexity:** O(n·m) time, O(n) space: at most 1.5·10⁶ relaxations here.
