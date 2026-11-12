## Approach: one average, then one subtraction per friend

The fair share is `total / n`, where `total` is the sum of all payments. Friend `i` paid `paid[i]`, so their balance is `paid[i] − total / n`. The balances always add up to zero (up to rounding): what some are owed, the others owe.

```
share = sum(paid) / len(paid)
return [p - share for p in paid]
```

### Walkthrough

`paid = [30.0, 10.5, 20.25, 0.0]`: the total is 60.75 and the share is 15.1875, so the balances are 14.8125, −4.6875, 5.0625 and −15.1875.

### About precision

Money amounts like 10.1 have no exact binary representation, so the computed balances can be off in the last digits. That is why answers within 10⁻⁶ are accepted. Summing 2·10⁴ amounts of at most 10⁴ keeps the error far below that.

**Complexity:** O(n) time and O(n) for the answer.
