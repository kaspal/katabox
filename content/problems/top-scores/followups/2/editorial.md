## Idea

A min-heap of the top `k` (the live-leaderboard trick) cannot drop a game that leaves the
window, so count by value instead. Scores lie in 0..300, so keep `count[v]`, how many games
in the current window scored `v`. Moving the window one game adds 1 to the new score's
count and subtracts 1 from the leaving score's count. The `k`-th highest is the value
where the running total of counts from 300 downwards first reaches `k`. Scanning 301
counts per window is already fast enough; a Fenwick tree over the values finds it in
O(log 301) by searching for the `(w − k + 1)`-th smallest.

## Complexity

O(n · log V) time with a Fenwick tree (or O(n · V) with a plain scan), where V = 301, and
O(V) extra memory besides the output of `scores.length − w + 1` values.
