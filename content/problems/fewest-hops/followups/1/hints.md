Listing every route and keeping the shortest ones explodes: a mesh of a few dozen
stations can already have millions of fewest routes. What does a fewest route to a
station look like just before its last hop?
---
A fewest route to station `v` at distance `d` is a fewest route to some neighbour `u` at
distance `d − 1`, plus the hop `u → v`. So the number of routes to `v` is the sum of the
numbers of routes to those neighbours.
---
Run the base problem's breadth-first search from station `0` with a `ways` array
(`ways[0] = 1`). When a neighbour is found for the first time, set its distance and copy
`ways[u]`; when it is found again at the same distance `d[u] + 1`, add `ways[u]`. Reduce
modulo 1 000 003.
