Adding up every window of `k` days from scratch costs O(k) per window. What changes between one window and the next?
---
When the window moves one day to the right, one day enters and one day leaves.
---
Sum the first `k` days. Then for each next day `i`, add `sales[i]` and subtract `sales[i - k]`, keeping the largest total seen.
