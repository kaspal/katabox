## Approach: prefix sums

Compute running totals once: `prefix[0] = 0` and `prefix[i + 1] = prefix[i] + miles[i]`. Then the total of days `a..b` is the total up to `b` minus the total before `a`:

```
trip(a, b) = prefix[b + 1] − prefix[a]
```

### Walkthrough

`miles = [4, 0, 7, 3, 5]` gives `prefix = [0, 4, 4, 11, 14, 19]`. Trip [1, 3] is `prefix[4] − prefix[1] = 14 − 4 = 10`.

**Complexity:** O(n + q) time, O(n) extra space. Summing each trip directly is O(n·q), up to 10¹⁰ additions.
