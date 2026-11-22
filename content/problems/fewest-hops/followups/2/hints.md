Trying each link turned around, with a full search each time, costs
O(links · (n + links)): far too slow. Can a single search decide on the turned link as it
goes?
---
What matters at each moment is not which link you turned but whether you have already
used your one turn. Make that part of where you are: each station appears twice, "turn
still free" and "turn used".
---
Run a breadth-first search over pairs `(station, used)`, starting from `(0, free)`. A
link `[a, b]` in its own direction goes from `(a, s)` to `(b, s)`; walked backwards it
goes from `(b, free)` to `(a, used)` only. The answer is the smaller distance of the two
copies of station `n − 1` that were reached.
