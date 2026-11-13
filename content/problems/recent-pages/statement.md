An e-reader keeps the pages you opened recently in a small cache of `capacity` pages. Process the operations in order, where `ops[i]` is the name and `args[i]` the numbers:

- `"get"`, args `[page]`: **report** the page's stored value, or `-1` if it is not in the cache. A page that is found counts as used now.
- `"put"`, args `[page, value]`: store `value` for the page (replacing an older value) and count the page as used now.

If a `"put"` adds a new page to a full cache, the page that was **used least recently** is removed first.

Return the reports of the `"get"` operations, in order.

### Example 1

```
capacity = 2, ops = ["put", "get", "put", "put", "put", "get", "get"], args = [[5, 50], [5], [8, 80], [5, 55], [9, 90], [8], [5]]
Output: [50, -1, 55]
```

Updating page 5 makes it the most recent, so adding page 9 removes page 8.

### Example 2

```
capacity = 1, ops = ["put", "put", "get", "get"], args = [[1, 7], [2, 3], [1], [2]]
Output: [-1, 3]
```

### Example 3

```
capacity = 3, ops = ["get"], args = [[4]]
Output: [-1]
```

### Constraints

- 1 ≤ capacity ≤ 10⁵
- 1 ≤ ops.length ≤ 2·10⁵, args.length = ops.length
- 1 ≤ page ≤ 10⁹, 0 ≤ value ≤ 10⁹
