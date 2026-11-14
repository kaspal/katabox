## Approach: sparse table of power-of-two minimums

Scanning every stretch costs O(N) per question: with 5·10⁴ questions over a 10⁵-reading log that is up to 5·10⁹ steps. The log never changes, so spend O(N log N) once on a table that answers any question in O(1).

**The table.** Let `low[k][i]` be the lowest of the `2^k` readings starting at position `i` (0-based). Row 0 is the log itself, and each row comes from the one before by combining two halves:

```
low[k][i] = min(low[k−1][i], low[k−1][i + 2^(k−1)])     for 0 ≤ i ≤ N − 2^k
```

There are ⌊log₂ N⌋ + 1 ≤ 17 rows, each at most `N` long.

**A question.** For the stretch `l..r` (0-based) of length `len = r − l + 1`, take the largest `k` with `2^k ≤ len`. The two pieces `[l, l + 2^k)` and `(r − 2^k, r]` both lie inside the stretch and together cover it (each is at least half of it), so

```
answer = min(low[k][l], low[k][r − 2^k + 1])
```

**Why the overlap is fine:** the two pieces may share readings, but taking a minimum twice changes nothing (`min` is idempotent). That is what makes the sparse table work for minimum, maximum or gcd, and not for sums, where an overlap would count readings twice.

**Finding k:** `len.bit_length() − 1` (Python), `31 − Math.clz32(len)` (TypeScript), a precomputed table `lg[i] = lg[i / 2] + 1` (PHP), `usize::BITS − 1 − len.leading_zeros()` (Rust).

**Complexity:** O(N log N) time and memory to build, O(1) per question: O(N log N + Q) in all. Remember that the input positions are 1-based.

**Fast input and output:** read all tokens at once and print all answers in one write. In Python, build each row with `list(map(min, prev[:-half], prev[half:]))`-style slices to keep the loop in C.
