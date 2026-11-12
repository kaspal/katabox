## Approach: sort by a chunk key

Turn every label into a **key**: a list with one entry per chunk.

- A digit chunk becomes `(0, length without leading zeros, digits without leading zeros)`. Comparing these compares number values without converting to integers: a shorter number is smaller, and equal lengths compare digit by digit. `007` and `7` give the same entry.
- A letter chunk becomes `(1, letters)`, so digits come before letters and letters compare alphabetically.

Comparing two keys entry by entry is exactly the natural order, including "a label that runs out of chunks first comes first" (a shorter key that matches so far is smaller). Sort with a **stable** sort, so labels with equal keys keep their input order.

```
key(label) = [(0, len(n), n) for digit chunks, n = chunk.lstrip("0") ...
              (1, s)         for letter chunks]
return sorted(labels, key=key)      # stable
```

### Walkthrough

`box10` → `(1, box), (0, 2, 10)`; `box2` → `(1, box), (0, 1, 2)`; `box2a` → `(1, box), (0, 1, 2), (1, a)`. So box2 < box2a < box10. `apple` starts with `(1, apple)`, smaller than `(1, box)`.

**Complexity:** O(n log n) comparisons, each O(L), plus O(n · L) to build the keys.
