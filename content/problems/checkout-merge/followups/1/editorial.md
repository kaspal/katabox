## Idea

Do the base problem's k-way merge and filter as you go. Put the head of every non-empty
line into a min-heap keyed by `(value, line index)`. Repeatedly pop the smallest node and
push its `next` if there is one. Because the merge emits values in non-decreasing order,
all copies of a number come out consecutively, so append a new node to the result only
when the value differs from the last one appended. A dummy head node keeps the appending
simple; return its `next` (empty when there were no customers).

## Complexity

O(N log k) time for `N` customers in `k` lines, and O(k) extra memory for the heap plus the
output list. Collecting every number into a set and sorting also works, in O(N log N).
