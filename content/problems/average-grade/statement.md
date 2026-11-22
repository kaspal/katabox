Ms. Okafor wrote a tiny helper to compute the class average for her report cards. Some students noticed that their averages looked suspiciously round, and a few were a fraction of a point lower than they expected.

Given an array `grades` of integer scores, return their average: the sum of all grades divided by how many there are. The result is a decimal number; answers within 10⁻⁶ of the exact value are accepted.

The editor starts with that code. Find the bug and fix it.

### Example 1

```
grades = [70, 85, 92]
Output: 82.33333333333333
```

(70 + 85 + 92) / 3 = 247 / 3.

### Example 2

```
grades = [72, 97, 8, 32]
Output: 52.25
```

### Example 3

```
grades = [11, 10, 46, 21]
Output: 22.0
```

### Constraints

- 1 ≤ grades.length ≤ 10⁵
- 0 ≤ grades[i] ≤ 100
