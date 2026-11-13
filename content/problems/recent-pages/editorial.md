## Approach: a hash map plus a recency order

Two operations must be O(1): find a page by its number, and keep the pages in order of last use so that the least recent one can be removed.

- A **hash map** from page to its entry finds pages.
- A **doubly linked list** (or an insertion-ordered map) keeps the order: a used page is unlinked and put at the "recent" end; eviction removes from the "old" end.

For `"get"`: if the page is present, move it to the recent end and report its value; otherwise report −1. For `"put"`: if present, update and move; if not, add it at the recent end, and if the cache now holds more than `capacity` pages, remove the oldest.

The references use what each language offers: Python's `OrderedDict` (`move_to_end`, `popitem(last=False)`), JavaScript's `Map` and PHP's arrays (both keep insertion order: delete and re-insert to refresh, the first key is the oldest), and in Rust a map plus a `BTreeMap` ordered by a use counter.

**Complexity:** O(1) per operation (O(log n) for the Rust ordered map). Searching a plain list is O(capacity) per operation.
