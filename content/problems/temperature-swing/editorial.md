## Approach: one pass, two trackers

Keep the warmest and the coldest reading seen so far. Both start at the first reading; every other reading may replace one of them. At the end, the swing is `warmest − coldest`.

With a single reading, both trackers hold it and the swing is 0.

**Complexity:** O(n) time, O(1) extra space. Most languages also have built-ins (`max`, `min`) that do the same two passes.
