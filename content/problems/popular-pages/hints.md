Scanning every cached page to find the one to remove costs O(capacity) per removal. The removal rule sorts pages by (use count, last use). How can you keep that order cheaply?
---
Group the pages by use count. Inside one group, keep them in order of last use (oldest first), like the recency list of a simple cache. A page that gets used moves from group `c` to the end of group `c + 1`.
---
Keep a map from count to an insertion-ordered set of pages, plus `minCount`, the smallest non-empty count. New pages set `minCount = 1`; when a page leaves the `minCount` group and it becomes empty, `minCount` grows by one. To evict, remove the oldest page of the `minCount` group.
