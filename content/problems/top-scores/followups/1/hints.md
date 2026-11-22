Sorting the games again after every new one costs O(n² log n). Which scores can never
become the `k`-th best again once they drop out of the top `k`?
---
A score that falls out of the top `k` stays out: new games only push it further down. So
you only need to keep the current top `k`, and the `k`-th best is the smallest of them.
---
Keep a min-heap of at most `k` scores. Push while it holds fewer than `k`; after that,
replace the top when a new score beats it. After each game, report the top of the heap
once it holds `k` scores, `-1` before that.
