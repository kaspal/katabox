## Approach: cheapest first

If some set of `k` snacks fits the budget, then the `k` cheapest snacks fit too: they cost no more. So the answer is the largest `k` for which the `k` cheapest prices add up to at most `budget`.

```
sort prices
spent = 0; count = 0
for p in prices:
    if spent + p > budget: break
    spent += p; count += 1
return count
```

Stopping at the first price that does not fit is safe: every later price is at least as high.

### Walkthrough

`[120, 35, 80, 60, 200]` sorted is `[35, 60, 80, 120, 200]`. Running totals: 35, 95, 175, then 295 > 180, so the answer is 3.

**Complexity:** O(n log n) for sorting, then O(n). Finding the cheapest remaining snack with a fresh scan for every purchase is O(n²).
