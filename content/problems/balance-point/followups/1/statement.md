The plank from the base problem (crate `i` pushes with force `weights[i]`; a negative
force is a helium balloon pulling up) is now held by **two** pivots, under two different
crates `i < j`. The pivots split the plank into three stretches: the crates left of `i`,
the crates strictly between `i` and `j`, and the crates right of `j`.

A pair of pivots balances the plank when all three stretches have the same total force.
The crates on the pivots count for no stretch, and an empty stretch totals 0 (so the
pivots may sit next to each other or at the very ends). Return **how many** pairs
`i < j` balance the plank.

### Example 1

```
weights = [2, 5, 2, 9, 1, 1]
Output: 1
```

Pivots at 1 and 3: the stretches are [2], [2] and [1, 1], each totalling 2.

### Example 2

```
weights = [2, -2, 0, -2, 1]
Output: 1
```

Pivots at 3 and 4: the left stretch is 2 + (−2) + 0 = 0, and the other two are empty.

### Example 3

```
weights = [-2, -2, 0, -1]
Output: 0
```

### Constraints

- 2 ≤ weights.length ≤ 10⁵
- −10⁴ ≤ weights[i] ≤ 10⁴
- The answer can exceed 32 bits (a plank of zeros balances on every pair); it fits in a
  64-bit integer and in a JavaScript number.
