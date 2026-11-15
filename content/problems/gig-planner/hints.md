Sort the offers by their end time. Then think about the last offer in that order: either the best plan leaves it out, or it takes it. If it takes it, what can the rest of the plan contain?
---
Let `best[i]` be the best plan using only the first `i` offers by end time. Taking offer `i` leaves room only for offers that end by its `start`, and in sorted order those form a prefix you can find with a binary search. Keep the tie-break inside the value: compare plans as the pair `(total, −count)`, where larger is better.
---
Sort by `end` and keep the sorted `ends`. `best[0] = (0, 0)`. For offer `i` (0-based) with `start` and `pay`, let `j` be the number of offers among the first `i` with `end ≤ start` (`bisect_right(ends, start, 0, i)`). Then `best[i + 1] = max(best[i], (best[j].total + pay, best[j].minus_count − 1))`. The answer is `best[n].total` and `−best[n].minus_count`.
