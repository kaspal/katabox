A cheapest network never contains a cycle: dropping the dearest cable of a cycle keeps everything connected and saves money. So the answer uses exactly `n − 1` cables.
---
Consider the cables from cheapest to dearest. Lay a cable only if its two villages are not connected yet; otherwise it would close a cycle.
---
Use union-find to answer "are these two villages connected yet?" in almost O(1). Sort the cables by cost (Kruskal's algorithm). If fewer than `n − 1` cables were laid at the end, return `-1`.
