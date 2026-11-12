Call a pair of children "out of order" when the taller one stands to the left of the shorter one. How does one swap of neighbours change the number of out-of-order pairs?
---
A swap of two neighbours changes the order of exactly that one pair, so it removes at most one out-of-order pair, and swapping an out-of-order neighbouring pair (as bubble sort does) removes exactly one. The answer is the number of out-of-order pairs. Counting them pair by pair is O(n²).
---
Count during **merge sort**: when merging two sorted halves and an element of the right half is placed before the remaining `k` elements of the left half, it was out of order with exactly those `k` (all taller). Add `k` each time.
