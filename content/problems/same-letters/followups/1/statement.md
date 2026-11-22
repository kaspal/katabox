This time bag `b` keeps changing. Its tiles lie in a row, as the string `b`, and the player swaps them one at a time: change `i` replaces the tile at position `positions[i]` with the letter `letters[i]`. Changes are permanent and add up.

After each change, check whether bags `a` and `b` are twins: every letter appears the same number of times in both. Return one answer per change, in order. Bags with different numbers of tiles are never twins.

### Example 1

```
a = "tale", b = "late", positions = [0, 0, 3], letters = "plp"
Output: [false, true, false]
```

`b` becomes "pate", then "late" again, then "latp".

### Example 2

```
a = "acab", b = "bcaa", positions = [0, 3, 0], letters = "bbc"
Output: [true, false, false]
```

The first change puts a `b` where there already was one.

### Example 3

```
a = "aaab", b = "cbbca", positions = [4, 0, 4], letters = "cab"
Output: [false, false, false]
```

`b` has five tiles and `a` has four, so they are never twins.

### Constraints

- 1 ≤ a.length, b.length ≤ 10⁵
- 1 ≤ positions.length = letters.length ≤ 10⁵
- 0 ≤ positions[i] < b.length
- a, b and letters contain only lowercase English letters.
