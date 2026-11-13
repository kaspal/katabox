The shelf is two sorted runs: the moved block at the end holds the smallest numbers. If you split the shelf in the middle, what can you say about the two halves?
---
At least one half is sorted, and you can tell which: `shelf[lo] ≤ shelf[m]` means the left half `lo..m` is sorted; otherwise the right half `m..hi` is.
---
In the sorted half, a range check tells you whether `label` can be there. If yes, search that half; if not, search the other one. Stop when you find it or the range is empty.
