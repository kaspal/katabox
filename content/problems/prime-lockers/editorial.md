## Approach: one sieve, then a running count

Let `M` be the largest limit. The **sieve of Eratosthenes** marks the primes up to `M` in one go:

```
prime = [true] * (M + 1); prime[0] = prime[1] = false
for p = 2 while p * p <= M:
    if prime[p]:
        for m = p * p, p * p + p, ... <= M: prime[m] = false
```

Every composite `m` has a prime factor `p ≤ √m`, so it gets crossed out by that `p`, and starting at `p²` misses nothing. Then build the running count `count[x] = count[x − 1] + (1 if prime[x] else 0)` and answer each question with `count[L]`.

### Walkthrough

Up to 30 the sieve crosses out the multiples of 2 (from 4), 3 (from 9) and 5 (from 25). What is left is 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, so `count[30] = 10` and `count[10] = 4`.

**Complexity:** O(M log log M) for the sieve, O(M) for the count and O(1) per question, with O(M) memory. Trial division for every number up to `L`, for every question, costs about `q · L · √L` operations.
