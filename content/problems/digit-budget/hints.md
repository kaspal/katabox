Checking every ticket up to 10¹⁵ is impossible. Build the numbers digit by digit instead, from the most significant one.
---
While choosing digits from the left, what matters for the rest is: the position, the digit sum so far, and whether the number is still equal to `limit`'s prefix ("tight") or already smaller.
---
`count(pos, sum, tight)`: if tight, the next digit can go up to `limit`'s digit at `pos`, else up to 9. Memoise the non-tight states (they repeat a lot). Count numbers 0..limit with digit sum `total`; since total ≥ 1, the number 0 is never counted.
