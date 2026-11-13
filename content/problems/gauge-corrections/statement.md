A row of river gauges reports water levels: `readings[i]` for gauge `i` (from 0). Engineers keep sending corrections and questions, `ops[j]` with numbers `args[j]`:

- `"add"` with `[a, b, delta]`: every gauge from `a` to `b` (both included) was off by `delta`; add `delta` to each of them. `delta` may be negative.
- `"read"` with `[i]`: report the current level of gauge `i`.

Return the answers of the `"read"` operations, in order.

### Example 1

```
readings = [10, 20, 30, 40], ops = ["add", "read", "add", "read", "read"], args = [[1, 3, 5], [2], [0, 1, -2], [1], [0]]
Output: [35, 23, 8]
```

Gauge 2 becomes 30 + 5 = 35. Gauge 1 gets +5 and then −2: 23.

### Example 2

```
readings = [0, 0], ops = ["read", "add", "read"], args = [[1], [0, 1, 7], [1]]
Output: [0, 7]
```

### Example 3

```
readings = [5], ops = ["add", "add", "read"], args = [[0, 0, 3], [0, 0, -9], [0]]
Output: [-1]
```

### Constraints

- 1 ≤ readings.length ≤ 10⁵
- -10⁴ ≤ readings[i] ≤ 10⁴
- 1 ≤ ops.length ≤ 10⁵, ops.length = args.length
- `"add"` has `[a, b, delta]` with 0 ≤ a ≤ b < readings.length and -100 ≤ delta ≤ 100; `"read"` has `[i]` with 0 ≤ i < readings.length.
