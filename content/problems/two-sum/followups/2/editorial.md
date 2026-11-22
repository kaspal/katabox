## Idea

A hash map answers "is the exact partner here?", not "which partner is nearest?", so
sort instead. With pointers `i` at the start and `j` at the end, look at
`s = a[i] + a[j]`. If `s < target`, every other partner of `a[i]` is smaller than
`a[j]` and gives an even smaller sum, so `a[i]` is done: move `i` right. If
`s > target`, `a[j]` is done in the same way: move `j` left. If `s == target`, nothing
can be closer. Every sum the pointers skip is further from `target` than one they
check, so keeping the best checked sum (ties go to the smaller sum) gives the answer.

## Complexity

O(n log n) time for the sort, then O(n) for the scan; O(n) extra memory for a sorted
copy (or O(1) if you sort in place). Sums reach 2·10⁹, past a 32-bit integer, so use
64-bit arithmetic.
