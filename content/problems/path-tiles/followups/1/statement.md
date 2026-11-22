The garden path is the same strip, 1 tile wide and `n` tiles long, paved end to end with
square tiles (1×1), short slabs (1×2) and long slabs (1×3), as many of each as needed. Two
pavings differ if the sequences of pieces from one end to the other differ.

This time the path is **very long**: `n` can be up to 10¹⁵ (it fits in a 64-bit integer
and in a JavaScript number). Return the number of pavings modulo **1 000 003**.

### Example 1

```
n = 3
Output: 4
```

Square-square-square, square-short, short-square, or one long slab.

### Example 2

```
n = 5
Output: 13
```

### Example 3

```
n = 4
Output: 7
```

### Constraints

- 1 ≤ n ≤ 10¹⁵
