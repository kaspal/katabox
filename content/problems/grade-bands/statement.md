A teacher turns exam scores (0 to 100) into letter bands for the report cards:

| Score | Band |
| --- | --- |
| 90 to 100 | `"A"` |
| 80 to 89 | `"B"` |
| 70 to 79 | `"C"` |
| 60 to 69 | `"D"` |
| below 60 | `"F"` |

Given the class's `scores`, return the band of each score, in the same order.

### Example 1

```
scores = [95, 82, 71, 64, 30]
Output: ["A", "B", "C", "D", "F"]
```

### Example 2

```
scores = [90, 89, 60, 59]
Output: ["A", "B", "D", "F"]
```

The edges of a band belong to it: 90 is an A, 60 is a D.

### Example 3

```
scores = [100, 0]
Output: ["A", "F"]
```

### Constraints

- 1 ≤ scores.length ≤ 10⁴
- 0 ≤ scores[i] ≤ 100
