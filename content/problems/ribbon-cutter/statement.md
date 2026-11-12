A gift-wrapping stall has rolls of ribbon with lengths `ribbons` (in centimetres). For a big order it needs at least `pieces` pieces of ribbon, **all of the same whole-number length**. Each roll can be cut into as many pieces of that length as fit; leftovers are thrown away, and pieces cannot be glued together.

Return the greatest length (at least 1) that yields at least `pieces` pieces, or `0` if even pieces of length 1 are not enough.

### Example 1

```
ribbons = [30, 47, 18], pieces = 6
Output: 15
```

Length 15 gives 2 + 3 + 1 = 6 pieces. Length 16 gives only 1 + 2 + 1 = 4.

### Example 2

```
ribbons = [5, 5], pieces = 11
Output: 0
```

Even 1 cm pieces make only 10.

### Example 3

```
ribbons = [100], pieces = 1
Output: 100
```

### Constraints

- 1 ≤ ribbons.length ≤ 10⁵
- 1 ≤ ribbons[i] ≤ 10⁷
- 1 ≤ pieces ≤ 10⁹
