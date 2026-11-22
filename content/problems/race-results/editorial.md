## The bug

The code sorts twice, relying on Python's sort being stable: first by name, then by time, so runners with equal times keep the name order from the first pass. The first pass is the problem:

```python
order = sorted(range(len(names)), key=lambda i: names[i], reverse=True)
```

`reverse=True` puts the names in **descending** order, so every tie ends up reverse-alphabetical. Inputs with all-different times are unaffected, which is why the bug is easy to miss. With `names = ["ana", "bo"]` and `times = [5, 5]`, the code returns `["bo", "ana"]` instead of `["ana", "bo"]`. Example 2 returns `["kim", "joe", "eli", "ben"]`.

## The fix

Sort the names in ascending order before the stable sort by time:

```python
def race_order(names: list[str], times: list[int]) -> list[str]:
    order = sorted(range(len(names)), key=lambda i: names[i])
    order.sort(key=lambda i: times[i])
    return [names[i] for i in order]
```

A single sort with the key `(times[i], names[i])` works too. In TypeScript, swap the tie-break so it reads `names[a] < names[b] ? -1 : 1`.

**Complexity:** O(n log n) time (times the name length for comparisons), O(n) extra space.
