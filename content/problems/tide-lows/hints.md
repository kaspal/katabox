Scanning each stretch costs up to `N` steps per question, and the stretches can be long. The log never changes: what could you work out once, before the questions, so that each answer is quick?
---
The lowest level of a stretch can be taken from two pieces that cover it, **even if the pieces overlap**: a reading counted twice does not change a minimum. Precompute the minimum of every piece whose length is a power of two.
---
Sparse table: `low[0] = levels`, and `low[k][i] = min(low[k−1][i], low[k−1][i + 2^(k−1)])` is the lowest of the `2^k` readings starting at `i`. For a question, let `k` be the largest power with `2^k ≤ r − l + 1`; the answer is `min(low[k][l], low[k][r − 2^k + 1])` (0-based positions).
