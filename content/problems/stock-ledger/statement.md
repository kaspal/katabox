A shop keeps a ledger of how many items sit on each shelf: `stock[i]` for shelf `i` (from 0). The day brings a list of operations, `ops[j]` with its numbers `args[j]`:

- `"set"` with `[i, count]`: shelf `i` now holds `count` items.
- `"total"` with `[a, b]`: report how many items are on shelves `a` to `b`, both included.

Return the answers of the `"total"` operations, in order.

### Example 1

```
stock = [5, 3, 8, 2], ops = ["total", "set", "total", "total"], args = [[0, 3], [2, 1], [1, 2], [2, 2]]
Output: [18, 4, 1]
```

All shelves hold 18 items. After shelf 2 drops to 1, shelves 1 and 2 hold 3 + 1 = 4.

### Example 2

```
stock = [0, 0, 0], ops = ["set", "set", "total"], args = [[0, 7], [2, 4], [0, 2]]
Output: [11]
```

### Example 3

```
stock = [6], ops = ["total"], args = [[0, 0]]
Output: [6]
```

### Constraints

- 1 ≤ stock.length ≤ 10⁵
- 0 ≤ stock[i], count ≤ 10⁴
- 1 ≤ ops.length ≤ 10⁵, ops.length = args.length
- `"set"` has `[i, count]` with 0 ≤ i < stock.length; `"total"` has `[a, b]` with 0 ≤ a ≤ b < stock.length.
