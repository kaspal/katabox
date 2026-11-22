## Idea

The base problem needed one recency order; here the victim is the least used page, with
recency only breaking ties. Keep three things: a map from page to value and use count,
one recency-ordered list per use count (a linked list with a map to its nodes, or an
insertion-ordered hash map), and `least`, the smallest count that has any page.

A use moves the page from list `f` to the end of list `f + 1`; if list `f` becomes empty
and `f` was `least`, then `least = f + 1` (the page just moved there). A new page goes to
the end of list 1 and sets `least = 1`, after evicting, if the cache is full, the first
(oldest) page of list `least`. Only a new page can lower `least`, and it lowers it to 1,
so `least` never needs a search.

## Complexity

O(1) per operation (expected, with hash maps), O(ops.length) in total, and
O(capacity) extra memory.
