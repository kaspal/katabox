## Idea

First undo the rotation without moving anything: the smallest folder sits where the
shelf "drops", and the base problem's binary search finds it. Compare `shelf[mid]` with
`shelf[hi]`; if it is larger, the drop is to the right of `mid`. Call that index `p`; then
`s(i) = shelf[(p + i) mod n]` reads the shelf in sorted order.

Below `s(i)` there are `s(i) − 1` positive numbers, and exactly `i` of them are on the
shelf, so `free(i) = s(i) − (i + 1)` of them are free. `free` never decreases (consecutive
folders differ by at least 1). For a query `k`, binary search the first `i` with
`free(i) ≥ k`. The `k`-th free number lies below `s(i)` and above `s(i − 1)`, with exactly
`i` folders below it, so it is `k + i`. If no `i` qualifies, all `n` folders lie below it
and the answer is `k + n`.

## Complexity

O(log n) for the rotation point and O(log n) per query: O(n + q log n) time with the input,
O(1) extra memory besides the output. A hash set of the shelf does not help here: the
questions are about numbers that are *not* on the shelf.
