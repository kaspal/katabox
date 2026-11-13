Adding `delta` to every gauge of a long range is the slow part. Can a range update touch only its two ends?
---
Think of a difference array: adding `delta` to gauges `a..b` is `+delta` at `a` and `−delta` at `b + 1`. A gauge's total correction is the prefix sum of the differences up to it.
---
Keep the differences in a Fenwick tree: `"add"` is two point updates, and `"read"` is `readings[i] + prefix(i + 1)`.
