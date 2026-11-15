## Approach: sort each section, take the lower middle

**Reading.** The input holds many sections, each a count followed by that many heights. Read **everything at once and split on whitespace**, then walk the tokens with a position `pos`: `T = tokens[0]`; for each section, `n = tokens[pos]` and the heights are the next `n` tokens; move `pos` past them.

**The median.** Sort the section's heights. With 0-based positions, the singer in the middle is at

```
(n − 1) / 2     (integer division)
```

- `n = 5`: position 2, the third of five, with two singers on each side.
- `n = 4`: position 1. The two middle singers are at positions 1 and 2, and the lower of them comes first in sorted order.
- `n = 1`: position 0, the only singer.

**Why it is correct:** after sorting, exactly `(n − 1) / 2` singers stand before position `(n − 1) / 2`. For odd `n` that leaves the same number after it, which is the middle. For even `n` it leaves one more after it than before, which makes it the first of the two middle singers, and in sorted order the first is the lower one. Repeated heights change nothing: equal values sit next to each other, and the one at that position is still the answer.

**Complexity:** O(n log n) per section for the sort, O(Σn log n) in all, with Σn ≤ 10⁵.

**Language traps:**

- TypeScript's `Array.prototype.sort()` with no argument compares **as strings**, so `[9, 10]` sorts as `[10, 9]`. Convert the tokens with `Number` and pass `(a, b) => a - b`, or use a `Float64Array`/`Int32Array`, whose `sort()` is numeric. Use `Math.floor((n - 1) / 2)` or `(n - 1) >> 1` for the integer division.
- PHP: cast the tokens to `int` before `sort()`; `intdiv($n - 1, 2)` gives the position.
- Rust: `sort_unstable()` on a `Vec<u32>` or `Vec<i64>`.

**Fast input and output:** read all of stdin once, collect the `T` answers and print them in one write.
