A row of crates sits on a long plank. Crate `i` pushes the plank with force `weights[i]`; a negative force means a helium balloon is tied to that crate and pulls it up.

You want to put the pivot under one crate so that the total force on its left equals the total force on its right (the crate on the pivot counts for neither side; an empty side totals 0).

Return the **smallest** index where the pivot balances the plank, or `-1` if there is none.

### Example 1

```
weights = [4, -1, 6, 2, 1]
Output: 2
```

Left of index 2: 4 + (-1) = 3. Right of it: 2 + 1 = 3.

### Example 2

```
weights = [5, 3, -3]
Output: 0
```

Nothing is left of index 0 (total 0), and 3 + (-3) = 0 on the right.

### Example 3

```
weights = [2, 7]
Output: -1
```

### Constraints

- 1 ≤ weights.length ≤ 10⁵
- −10⁴ ≤ weights[i] ≤ 10⁴
