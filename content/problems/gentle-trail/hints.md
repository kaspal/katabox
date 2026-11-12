For a fixed limit `L`, you can check whether some trail never takes a step harder than `L`: search the grid, using only steps of at most `L`. The answer is the smallest `L` for which the end is reachable.
---
Trying every limit one by one is slow, but the check is monotone: if `L` works, any bigger limit works. That allows a binary search on `L`. There is also a direct way.
---
Run Dijkstra's algorithm where a trail's "length" is its hardest step: from a spot reached with hardest step `d`, a neighbour is reached with `max(d, |difference|)`. A min-heap always expands the spot with the smallest `d`; the first time the bottom-right spot comes out, its `d` is the answer.
