A network of weather stations along a valley forecasts the night temperature at each station: `temps[i]` for station `i` (from 0). Forecasters keep revising and asking, `ops[j]` with numbers `args[j]`:

- `"shift"` with `[a, b, delta]`: every forecast from station `a` to `b` (both included) changes by `delta`, which may be negative.
- `"coldest"` with `[a, b]`: report the lowest current forecast among stations `a` to `b`.

Return the answers of the `"coldest"` operations, in order.

### Example 1

```
temps = [5, -2, 7, 0, 3], ops = ["coldest", "shift", "coldest", "coldest"], args = [[0, 4], [1, 3, 4], [1, 3], [2, 4]]
Output: [-2, 2, 3]
```

After shifting stations 1 to 3 by +4, the forecasts are [5, 2, 11, 4, 3].

### Example 2

```
temps = [1, 1, 1], ops = ["shift", "shift", "coldest"], args = [[0, 1, -5], [1, 2, -5], [0, 2]]
Output: [-9]
```

Station 1 is shifted twice: 1 − 5 − 5 = −9.

### Example 3

```
temps = [8], ops = ["coldest"], args = [[0, 0]]
Output: [8]
```

### Constraints

- 1 ≤ temps.length ≤ 10⁵
- -1000 ≤ temps[i] ≤ 1000
- 1 ≤ ops.length ≤ 10⁵, ops.length = args.length
- `"shift"` has `[a, b, delta]` with 0 ≤ a ≤ b < temps.length and -50 ≤ delta ≤ 50; `"coldest"` has `[a, b]` with 0 ≤ a ≤ b < temps.length.
