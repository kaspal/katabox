## Approach: test every year

Loop over the years from `start` to `end` and count those that satisfy

```
y % 4 == 0 and (y % 100 != 0 or y % 400 == 0)
```

The parentheses matter: the exception for multiples of 100 only applies to years divisible by 4, and the exception to the exception (400) only to multiples of 100.

**Complexity:** O(end − start) time, at most 9999 checks.

**Going further:** the number of leap years up to `y` is `y/4 − y/100 + y/400` (integer division), so the answer is `L(end) − L(start − 1)` in O(1).
