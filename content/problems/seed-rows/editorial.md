## Approach: dynamic programming over (length filled, last kind)

Let `ways[x][k]` count the valid plantings of exactly `x` centimetres whose **last** plant is of kind `k`, and `total[x] = Σ ways[x][k]`, with `total[0] = 1` for the empty bed (and `ways[0][k] = 0`).

Take a planting counted by `ways[x][k]` and remove its last plant. What is left fills `x − w` centimetres (`w = widths[k]`), is itself valid, and does not end with kind `k`, because that would put two `k` together. Conversely, appending a kind-`k` plant to any such shorter planting gives a planting counted by `ways[x][k]`. Hence

```
ways[x][k] = total[x − w] − ways[x − w][k]      (0 when x < w)
total[x]   = Σ over k of ways[x][k]
```

The empty bed has no last plant, so `total[0] − ways[0][k] = 1` correctly allows a planting made of one plant. Compute `x` from 1 to `length`; the answer is `total[length]`. Keep every number modulo 1 000 000 007 (add the modulus before taking the difference so it stays non-negative).

### Walkthrough

`widths = [1, 2]` (A, B). `total[0] = 1`. x = 1: A = 1, B = 0, total 1. x = 2: A = total[1] − A[1] = 0, B = total[0] − 0 = 1, total 1. x = 3: A = 1 − 0 = 1 (BA), B = 1 − 0 = 1 (AB), total 2. x = 4: A = 2 − 1 = 1 (ABA), B = 1 − 1 = 0, total 1.

**Complexity:** O(length · k) time and memory (or O(maxWidth · k) memory with a rolling window). Trying every planting recursively takes exponential time.
