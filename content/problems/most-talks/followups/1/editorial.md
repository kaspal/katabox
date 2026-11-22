## Idea

With scores, the earliest-end greedy of the base problem fails, so use dynamic
programming over the talks sorted by end. Let `best[i]` be the best total score using only
the first `i` talks in that order (`best[0] = 0`). Talk `i` with `[s, e, score]` is either
skipped, giving `best[i]`, or taken. If taken, every other chosen talk must end at or
before `s`; since the ends are sorted, those talks are a prefix, and a binary search
(upper bound of `s` among the earlier ends) gives its length `j`. So
`best[i + 1] = max(best[i], best[j] + score)`, and the answer is the last entry.

## Complexity

O(n log n) time for the sort and the binary searches, O(n) extra memory. The total is at
most 10⁵ · 10⁴ = 10⁹, which fits in a 32-bit integer.
