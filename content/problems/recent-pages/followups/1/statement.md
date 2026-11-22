The e-reader's cache now keeps the pages you **open most often**. The operations are the
same as before, where `ops[i]` is the name and `args[i]` the numbers:

- `"get"`, args `[page]`: **report** the page's stored value, or `-1` if it is not in the
  cache. A page that is found counts as one more use.
- `"put"`, args `[page, value]`: store `value` for the page (replacing an older value). A
  page already in the cache counts as one more use; a new page starts with one use.

What changes is the eviction. If a `"put"` adds a new page to a full cache, the page with
the **fewest uses** is removed first; among pages with equally few uses, the one **used
least recently** goes. A removed page forgets its uses: if it comes back, it starts again
at one. A `"get"` for a missing page changes nothing.

Return the reports of the `"get"` operations, in order.

### Example 1

```
capacity = 2, ops = ["put", "put", "get", "get", "put", "get", "get"], args = [[7, 70], [4, 40], [4], [7], [9, 90], [4], [9]]
Output: [40, 70, -1, 90]
```

Pages 7 and 4 both have two uses when page 9 arrives, and page 4 was used less recently, so
it is the one removed.

### Example 2

```
capacity = 2, ops = ["get", "put", "put", "get", "get", "put"], args = [[1], [4, 483], [4, 96], [4], [4], [1, 272]]
Output: [-1, 96, 96]
```

### Example 3

```
capacity = 2, ops = ["get", "put", "get", "put", "put", "put"], args = [[1], [2, 685], [3], [2, 595], [2, 653], [4, 880]]
Output: [-1, -1]
```

Updating page 2 does not add a page, so page 4 fits without removing anything.

### Constraints

- 1 ≤ capacity ≤ 10⁵
- 1 ≤ ops.length ≤ 2·10⁵, args.length = ops.length
- `ops[i]` is `"get"` (args `[page]`) or `"put"` (args `[page, value]`)
- 1 ≤ page ≤ 10⁹, 0 ≤ value ≤ 10⁹
