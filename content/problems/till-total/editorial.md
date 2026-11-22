## The bug

The starter adds the prices as floating-point numbers and converts at the end:

```python
total = 0.0
for p in prices:
    total += float(p)
return int(total * 100)
```

Binary floating point cannot store most two-decimal values exactly. `4.35` is really stored as 4.34999999999999964..., so `total * 100` is a hair below the true number of cents, and `int` cuts the fraction off instead of rounding. The sum can also pick up small errors along the way.

With `prices = ["4.35"]`, `float("4.35") * 100` is `434.99999999999994`, so the code returns `434` instead of `435`.

## The fix

Never leave whole numbers. Every price has exactly two decimals, so removing the dot gives its value in cents, and integer addition is exact:

```python
return sum(int(p.replace(".", "")) for p in prices)
```

In TypeScript, add `Number(p.replace(".", ""))` for each price and return the sum; the largest possible total, about 10¹⁰ cents, is still an exact integer in a JavaScript number. (Rounding with `Math.round(total * 100)` happens to patch these inputs, but counting in cents is exact by construction.)

**Complexity:** O(L) time, where L is the total length of the strings, and O(1) extra space.
