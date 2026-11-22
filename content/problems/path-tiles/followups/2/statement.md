The garden path is the same strip, 1 tile wide and `n` tiles long, paved end to end with
square tiles (1×1), short slabs (1×2) and long slabs (1×3). Two pavings differ if the
sequences of pieces from one end to the other differ.

This time two **long slabs must never touch**: a long slab may not be followed directly
by another long slab. Count the pavings that respect this rule and return the number
modulo 1 000 000 007.

### Example 1

```
n = 6
Output: 23
```

Of the 24 pavings without the rule, only long-long puts two long slabs together.

### Example 2

```
n = 5
Output: 13
```

Two long slabs need 6 tiles, so for `n = 5` the rule removes nothing.

### Example 3

```
n = 4
Output: 7
```

### Constraints

- 1 ≤ n ≤ 10⁶
