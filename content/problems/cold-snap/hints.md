Both operations touch a whole range. A structure that splits the stations into nested segments, each knowing its own minimum, can answer a range with O(log n) segments.
---
That is a segment tree. A `"shift"` of a whole segment changes its minimum by `delta` too, so you can record the shift at the segment instead of visiting every station.
---
Store at each node `min` (of its segment, with its own pending shift included) and `pending` (a shift not yet passed to its children). Push the pending shift down before visiting the children; after updating them, `min = min(left.min, right.min)`.
