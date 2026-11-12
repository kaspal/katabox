A garden path is a strip 1 tile wide and `n` tiles long. The gardener has square tiles (1×1), short slabs (1×2) and long slabs (1×3), as many of each as needed, and lays them end to end along the path with no gaps and no overlaps.

In how many different ways can the path be paved? Two pavings differ if the sequences of pieces from one end to the other differ. The number grows fast: return it modulo 1 000 000 007.

### Example 1

```
n = 3
Output: 4
```

Square-square-square, square-short, short-square, or one long slab.

### Example 2

```
n = 1
Output: 1
```

### Example 3

```
n = 5
Output: 13
```

### Constraints

- 1 ≤ n ≤ 10⁶
