## Idea

Simulate the rule with two heaps. Sort the indices by `(start, index)` so ties follow the
input order. Keep `busy`, a min-heap of `(end, room)`, and `free`, a min-heap of room
numbers. For each meeting, first pop every busy room whose end is at most the meeting's
start and push its number into `free`; a room freed exactly at the start counts. Then give
the meeting the top of `free`, or a brand-new room numbered one past the rooms opened so
far when `free` is empty, store it at the meeting's original index, and push
`(end, room)` into `busy`. The largest number handed out equals the base problem's answer.

## Complexity

O(n log n) time: the sort, plus at most one push and one pop per meeting in each heap.
O(n) extra memory for the order, the heaps and the output.
