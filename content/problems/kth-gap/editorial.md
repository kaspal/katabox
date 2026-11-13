## Approach: binary search on the answer, counting with two pointers

Sort the posts. Let `count(d)` be the number of pairs whose gap is at most `d`. It never decreases as `d` grows, and the k-th gap is the smallest `d` with `count(d) ≥ k`.

**Counting in O(n):** for each `j`, move `i` forward while `posts[j] − posts[i] > d`; then the posts `i..j−1` are all within `d` of post `j`, which adds `j − i` pairs. `i` never moves back, so the whole count is O(n).

**Searching:** binary search `d` over `[0, posts[n−1] − posts[0]]`: if `count(mid) ≥ k`, the answer is at most `mid`; otherwise it is larger. About 20 rounds cover 10⁶.

### Walkthrough

Sorted posts `[2, 3, 7, 10]`, k = 4. count(4) = 3 (gaps 1, 3, 4), count(5) = 4. So the answer is 5.

**Complexity:** O(n log n + n log W) time, where W is the largest gap; O(1) extra space after sorting. Listing every gap is O(n²), over 10⁹ pairs.
