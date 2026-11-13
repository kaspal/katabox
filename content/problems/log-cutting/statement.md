A sawmill must cut a log of length `length` at every position in `marks` (measured from the left end). Each cut is made on one piece at a time, and the saw charges the **current length of the piece** being cut. The cuts can be made in any order.

Return the smallest total charge for making all the cuts.

### Example 1

```
length = 10, marks = [2, 4, 7]
Output: 20
```

Cutting at 4 first costs 10; then 2 on the piece 0–4 costs 4, and 7 on the piece 4–10 costs 6: 20. Cutting at 2, then 4, then 7 would cost 10 + 8 + 6 = 24.

### Example 2

```
length = 5, marks = [1]
Output: 5
```

### Example 3

```
length = 12, marks = [3, 8, 10]
Output: 24
```

### Constraints

- 2 ≤ length ≤ 10⁶
- 1 ≤ marks.length ≤ 100
- 0 < marks[i] < length, all different
