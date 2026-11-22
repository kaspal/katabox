Plain Dijkstra finds the cheapest route but ignores how many roads it uses, and a cheap
route with too many roads is no good. What extra information does each town need?
---
Think in rounds: if you know the cheapest toll to every town using at most `r` roads, how
do you get the cheapest toll using at most `r + 1`?
---
Run `k` rounds of relaxation over every road, in both directions. In each round, read the
costs from the previous round and write into a copy, so one round never chains two roads.
Stop early if a round changes nothing.
