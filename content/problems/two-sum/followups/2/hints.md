A hash map finds an exact partner, but here the best partner may be missing. What order
of the numbers would make "a slightly bigger sum" or "a slightly smaller sum" easy to
reach?
---
Sort the numbers and look at the smallest and the largest together. If their sum is
below `target`, can the smallest number ever do better with any other partner?
---
Keep one pointer at each end of the sorted array. Record the sum when it beats the best
so far (closer, or equally close and smaller), then move the left pointer right if the
sum is too small, the right pointer left if it is too big, and stop on an exact hit.
