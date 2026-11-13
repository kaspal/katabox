## Approach: buckets by use count, each in recency order

The page to remove is the minimum of `(use count, last use)`. Keep:

- `value[page]` and `count[page]`;
- `bucket[c]`: the pages with use count `c`, in order of last use, oldest first (an insertion-ordered set, or a linked list);
- `minCount`: the smallest `c` with a non-empty bucket.

**Using a page** (a found `"get"`, or a `"put"` on a cached page): remove it from `bucket[c]`, append it to `bucket[c + 1]`, and if `bucket[c]` became empty and `c = minCount`, set `minCount = c + 1`. Appending at the end keeps each bucket in recency order.

**Adding a page** to a full cache: remove the first (oldest) page of `bucket[minCount]`. Then add the new page to `bucket[1]` and set `minCount = 1`.

Every step is O(1) with hash maps and ordered sets. (The Rust reference keeps an ordered set of `(count, last use, page)` instead, O(log n) per step, which is simpler to write in Rust.)

### Walkthrough

Example 1: after `put 3`, `put 4`, `get 3`, the buckets are {1: [4], 2: [3]}. `put 5` removes 4 (bucket 1). `get 5` moves 5 to bucket 2: {2: [3, 5]}, minCount 2. `put 6` removes 3, the oldest of bucket 2.

**Complexity:** O(1) per operation. Scanning all cached pages for every removal is O(capacity) per operation.
