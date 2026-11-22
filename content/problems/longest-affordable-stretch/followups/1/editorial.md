## Idea

Negative costs break the two-pointer window, so switch to prefix sums. Stages `i..j−1` cost
`prefix[j] − prefix[i]` and fit when `prefix[i] ≥ prefix[j] − budget`; for each end `j` the
best start is the earliest `i < j` that satisfies this. If an earlier start already has a
prefix at least as high as a later one, the later one never wins, so keep only the starts
that set a new record high of the prefix, starting with `prefix[0] = 0`. Their prefixes
are increasing, so a binary search finds the first one ≥ `prefix[j] − budget`, which is
also the earliest fitting start. Update the best length with `j − i`, then record `prefix[j]`
if it is a new high. If no start ever fits, the answer stays 0.

## Complexity

O(n log n) time and O(n) extra memory. Prefix sums stay within ±5·10⁸, so 32-bit integers
suffice, though `prefix[j] − budget` can reach about 1.5·10⁹.
