There can be over 10⁹ pairs, too many to list. Instead of finding the k-th gap directly, can you answer an easier question: "how many gaps are at most d?"
---
With the posts sorted, the posts within `d` of post `j` on its left form a contiguous block that only slides right as `j` grows: two pointers count all gaps ≤ d in O(n).
---
The count grows with `d`, so binary search on the answer: the smallest `d` in `[0, max − min]` whose count is at least `k` is the k-th gap.
