## The bug

The sliding window is built correctly, but the answer is recorded too early:

```python
        count[c] = count.get(c, 0) + 1
        best = max(best, end - start + 1)
        while len(count) > k:
```

Right after adding `text[end]`, the window may hold `k + 1` distinct letters. The starter measures it at that moment, before the `while` loop shrinks it back to a valid window, so an invalid window can become the answer.

With `text = "aab", k = 1`: after the second `a` the window is "aa" (length 2). Adding `b` makes it "aab", which has two letters, but `best` becomes 3 before the shrink removes the `a`s. The starter returns 3; the answer is 2. On Example 2 it returns 5 (from "bhhhg", which has three letters) instead of 4.

## The fix

Record the length only after the window is valid again:

```python
    for end, c in enumerate(text):
        count[c] = count.get(c, 0) + 1
        while len(count) > k:
            s = text[start]
            count[s] -= 1
            if count[s] == 0:
                del count[s]
            start += 1
        best = max(best, end - start + 1)
```

In TypeScript, move `best = Math.max(best, end - start + 1);` below the `while` loop in the same way.

**Complexity:** O(n) time, since each letter enters and leaves the window at most once; O(1) extra space (at most 27 letters in the map).
