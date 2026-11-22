## Idea

The count still follows `f(i) = f(i − 1) + f(i − 2) + f(i − 3)` with `f(0) = 1` (the
empty path), but `n` is far too large to step through. Write one step as a matrix:

```
[f(i+1)]   [1 1 1] [f(i)  ]
[f(i)  ] = [1 0 0] [f(i-1)]
[f(i-1)]   [0 1 0] [f(i-2)]
```

so `n` steps are that matrix raised to the `n`-th power, applied to `(1, 0, 0)`. Fast
exponentiation computes the power with about 2·log₂ n matrix products, each reduced
modulo 1 000 003. The small modulus keeps every entry below about 10⁶, so a product of
two entries stays below 10¹², and a sum of three of them is exact even in a JavaScript
number. One trap in JavaScript and TypeScript: the bit operators (`n & 1`, `n >> 1`) work
on 32 bits and would cut `n` short, so halve it with `n % 2` and `Math.floor(n / 2)`.

## Complexity

O(log n) time: about 50 squarings and up to 50 more products of 3×3 matrices for
n = 10¹⁵. O(1) extra memory.
