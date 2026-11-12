## Approach: Kruskal's algorithm with union-find

We want a **minimum spanning tree**: the cheapest set of cables that connects all villages. Kruskal's algorithm builds it greedily:

1. Sort the cables by cost.
2. Go through them in order. If the cable joins two villages that are not yet connected, lay it and merge their groups; if they are already connected, skip it (it would only close a cycle).
3. Stop after `n − 1` cables. If the list runs out first, some village cannot be reached: return `-1`.

Why the greedy choice is safe: take the cheapest cable `e` between the group containing village `a` and the rest. Any network that connects everything has some cable crossing between those two sides; swapping it for `e` keeps the network connected and costs no more. So some cheapest network contains `e`.

A **union-find** structure answers "same group?" and merges groups in almost constant time.

```
sort cables by cost
total = 0; laid = 0
for a, b, cost in cables:
    if union(a, b):          # true when a and b were in different groups
        total += cost; laid += 1
return total if laid == n - 1 else -1
```

### Walkthrough

Sorted: 1–2 (3), 2–3 (4), 0–2 (5), 0–1 (7), 1–3 (9). Lay 1–2, lay 2–3, lay 0–2: three cables for four villages, total 12. 0–1 and 1–3 would close cycles.

**Complexity:** O(m log m) time for sorting plus nearly O(m) for the unions, and O(n + m) space. Growing the network one village at a time while rescanning every cable for the cheapest one leaving it (a simple Prim's algorithm) is O(n · m).
