## Approach: two pointers with running maxima

The water above column `k` is `min(L[k], R[k]) − columns[k]`, where `L[k]` is the tallest column in `0..k` and `R[k]` the tallest in `k..n−1` (both include `k`, so the result is never negative).

Two prefix-maximum arrays give an O(n) time, O(n) space solution. Two pointers save the arrays:

- `i = 0`, `j = n − 1`, `leftMax = rightMax = 0`;
- while `i ≤ j`: update `leftMax` with `columns[i]` and `rightMax` with `columns[j]`;
  - if `leftMax ≤ rightMax`: column `i`'s right side has a column at least `rightMax ≥ leftMax` tall, so `min(L[i], R[i]) = leftMax`; add `leftMax − columns[i]`, then `i++`;
  - otherwise, symmetrically, add `rightMax − columns[j]`, then `j--`.

### Walkthrough

`[3, 0, 2, 0, 4]`: leftMax becomes 3 and stays below rightMax = 4, so the left pointer walks across: 3 − 3, 3 − 0, 3 − 2, 3 − 0, and finally the 4 itself adds 0. Total 7.

**Complexity:** O(n) time, O(1) space. Scanning both sides for every column is O(n²), 4·10¹⁰ steps for 2·10⁵ columns.
