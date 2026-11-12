The corner shop wants a tiny stock keeper. It starts empty and receives a list of operations: `ops[i]` is the operation's name and `args[i]` its numbers. Items are identified by a product code.

- `"stock"`, args `[item, qty]`: `qty` more units of `item` arrive.
- `"sell"`, args `[item, qty]`: a customer wants `qty` units of `item`. The shop sells as many as it has, up to `qty`. **Report** the number of units sold.
- `"count"`, args `[item]`: **report** how many units of `item` are left.

Return the reports in the order the operations happen (`"stock"` reports nothing).

### Example 1

```
ops = ["stock", "sell", "count", "sell", "count"], args = [[7, 5], [7, 3], [7], [7, 4], [7]]
Output: [3, 2, 2, 0]
```

Five units of item 7 arrive and 3 are sold, leaving 2. The next customer wants 4 but only 2 are left, so 2 are sold and none remain.

### Example 2

```
ops = ["count"], args = [[42]]
Output: [0]
```

An item that never arrived has 0 units.

### Example 3

```
ops = ["stock", "stock", "sell", "count", "count"], args = [[1, 10], [2, 4], [1, 10], [1], [2]]
Output: [10, 0, 4]
```

### Constraints

- 1 ≤ ops.length ≤ 10⁴, args.length = ops.length
- Each `ops[i]` is `"stock"`, `"sell"` or `"count"`, with the numbers described above
- 1 ≤ item ≤ 10⁹, 1 ≤ qty ≤ 10⁴
