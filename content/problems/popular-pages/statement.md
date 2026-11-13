A news site caches its pages, keeping the **popular** ones. The cache holds at most `capacity` pages, and each cached page has a **use count**. Process the operations in order, where `ops[i]` is the name and `args[i]` the numbers:

- `"get"`, args `[page]`: **report** the page's stored value, or `-1` if it is not in the cache. A page that is found gets one more use.
- `"put"`, args `[page, value]`: store `value` for the page. If the page is cached, its value is replaced and it gets one more use. If it is new, it enters with a use count of 1; but first, if the cache is full, the page with the **smallest use count** is removed, and among those, the one **used least recently**.

Return the reports of the `"get"` operations, in order.

### Example 1

```
capacity = 2, ops = ["put", "put", "get", "put", "get", "get", "put", "get", "get"], args = [[3, 30], [4, 40], [3], [5, 50], [4], [5], [6, 60], [3], [6]]
Output: [30, -1, 50, -1, 60]
```

Page 3 has 2 uses, so page 5 replaces page 4. Then pages 3 and 5 both have 2 uses; page 3 was used less recently, so page 6 replaces it.

### Example 2

```
capacity = 1, ops = ["put", "get", "put", "get", "get"], args = [[1, 5], [1], [2, 6], [1], [2]]
Output: [5, -1, 6]
```

### Example 3

```
capacity = 2, ops = ["put", "put", "put", "put", "get", "get"], args = [[1, 1], [1, 2], [2, 3], [3, 4], [1], [2]]
Output: [2, -1]
```

Updating page 1 gives it 2 uses, so page 3 replaces page 2.

### Constraints

- 1 ≤ capacity ≤ 10⁵
- 1 ≤ ops.length ≤ 2·10⁵, args.length = ops.length
- 1 ≤ page ≤ 10⁹, 0 ≤ value ≤ 10⁹
