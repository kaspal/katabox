This time shelves are not set one at a time: a **delivery** adds the same number of items
to every shelf in a row of consecutive shelves. `stock[i]` is the starting count of shelf
`i` (from 0), and the day brings operations `ops[j]` with their numbers `args[j]`:

- `"add"` with `[a, b, count]`: each of shelves `a` to `b`, both included, gets `count`
  more items.
- `"total"` with `[a, b]`: report how many items are on shelves `a` to `b`, both included.

Return the answers of the `"total"` operations, in order.

### Example 1

```
stock = [4, 0, 7, 1], ops = ["total", "add", "total", "total"], args = [[0, 3], [1, 2, 5], [0, 3], [2, 2]]
Output: [12, 22, 12]
```

The delivery puts 5 more items on shelves 1 and 2, so the shelves hold 4, 5, 12 and 1.

### Example 2

```
stock = [4, 18, 2, 8, 3], ops = ["add", "add", "total", "add", "total"], args = [[3, 3, 6], [0, 1, 6], [3, 4], [0, 3, 3], [0, 4]]
Output: [17, 65]
```

### Example 3

```
stock = [1, 2, 2, 11, 5], ops = ["total", "total", "total", "total", "add"], args = [[2, 2], [0, 4], [1, 3], [2, 4], [3, 4, 0]]
Output: [2, 21, 15, 18]
```

### Constraints

- 1 ≤ stock.length ≤ 10⁵
- 0 ≤ stock[i], count ≤ 10⁴
- 1 ≤ ops.length ≤ 10⁵, ops.length = args.length
- `"add"` has `[a, b, count]` and `"total"` has `[a, b]`, with 0 ≤ a ≤ b < stock.length.
- A total fits in a 64-bit integer (and in a JavaScript number).
