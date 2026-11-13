For one column, the water above it is `min(tallest on the left, tallest on the right) − height`, if that is positive. Scanning both sides for every column is O(n²).
---
The tallest-to-the-left values are a running maximum from the left, and the tallest-to-the-right values a running maximum from the right: two arrays, built in O(n).
---
To use O(1) space, move two pointers inwards with the running maxima `leftMax` and `rightMax`. If `leftMax ≤ rightMax`, the left column's water is decided by `leftMax` (something at least as tall waits on the right): add `leftMax − height` and move left. Otherwise do the same on the right.
