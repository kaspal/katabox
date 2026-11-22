## The bug

The search range is initialized one short:

```python
lo, hi = 0, len(sorted) - 1
```

The loop keeps the answer inside `[lo, hi]` and returns `lo` once the two meet, so the result is always between `0` and `hi`. Valid answers go from `0` to `len(sorted)` (the "nothing is big enough" case), and that last value is unreachable. With `sorted = [1, 2]` and `x = 3`, the loop checks index 0 (1 < 3, so `lo = 1`), stops because `lo == hi`, and returns `1` instead of `2`. Example 3 returns `4` instead of `5`.

## The fix

Let the range include the position just past the end:

```python
lo, hi = 0, len(sorted)
```

`mid` is still always less than `hi`, so `sorted[mid]` never goes out of bounds. In TypeScript, initialize `let hi = sorted.length;`.

**Complexity:** O(log n) time, O(1) extra space.
