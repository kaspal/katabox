## Approach: sort, fix one gift, sweep the other two

Sort the prices. Any trio, written in sorted order, has a cheapest gift at some index `i` and the other two at indices `lo < hi` after it. For a fixed `i`:

- start with `lo = i + 1` and `hi = n − 1`;
- if `s = p[i] + p[lo] + p[hi]` is below the target, every pair using this `lo` with a smaller `hi` is even further below, so move `lo` right;
- if `s` is above the target, by the same argument move `hi` left;
- if `s` equals the target, that is the answer.

Each sweep looks at every candidate that could be the best for this `i`, so keeping the best `s` over all `i` gives the answer. "Best" means a smaller `|s − target|`, and on a tie the smaller `s`.

```
sort p; best = p[0] + p[1] + p[2]
for i in 0 .. n-3:
    lo, hi = i + 1, n - 1
    while lo < hi:
        s = p[i] + p[lo] + p[hi]
        if better(s, best): best = s
        if s < target: lo += 1
        elif s > target: hi -= 1
        else: return s
return best
```

### Walkthrough

`prices = [10, 20, 30, 40]`, `target = 75`. With `i = 0` the sweep sees 70 (below: `lo` moves), then 80 (above: `hi` moves) and stops. Both are 5 away, so 70 is kept. With `i = 1` it sees 90, further away. Answer 70.

**Complexity:** O(n²) time after an O(n log n) sort, and O(1) extra space. Checking all triples is O(n³).
