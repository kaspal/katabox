Handle the fence first: a point is on a side when it is on the side's line (cross product zero) and within the side's bounding box.
---
For the other points, cast a ray from the point towards +x and count how many sides it crosses. An odd count means inside.
---
A side from (x1, y1) to (x2, y2) is crossed when exactly one end is above the point (`(y1 > y) != (y2 > y)`) and the crossing is to the right of the point. Compare with integers: the crossing is to the right when `(y − y1)(x2 − x1) − (x − x1)(y2 − y1)` has the same sign as `y2 − y1`.
