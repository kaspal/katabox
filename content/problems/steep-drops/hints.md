Checking every pair is about 2·10¹⁰ checks. Suppose the first half and the second half of the days were each sorted: could you count the steep drops **across** the halves quickly?
---
With both halves sorted, walk the first half in increasing order and move a pointer through the second half: the number of second-half prices `y` with `x > 2y` only grows as `x` grows.
---
Merge sort: count inside each half recursively, count across the halves with the moving pointer, then merge the halves so the caller gets a sorted list. Count before merging, because the merge loses which half a price came from.
