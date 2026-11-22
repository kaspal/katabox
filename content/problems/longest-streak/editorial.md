## The bug

The code records a run only when it ends because the next value is different:

```python
        else:
            best = max(best, run)
            run = 1
    return best
```

The last run in the array is never followed by a different value, so it is never compared with `best`. Whenever the longest run sits at the end, the answer is too small. With `values = [1, 2, 2]` the code returns `1` instead of `2`, and with `values = [5]` the loop does not run at all and it returns `0` instead of `1`. Example 1 returns `2` instead of `3`.

## The fix

Account for the final run when the loop finishes:

```python
    return max(best, run)
```

In TypeScript, the last line becomes `return Math.max(best, run);`.

**Complexity:** O(n) time, O(1) extra space.
