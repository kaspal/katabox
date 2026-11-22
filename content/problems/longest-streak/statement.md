Ravi logs the weather code for each day and wants to know the longest stretch of identical days. His function handles most logs, yet on some of them it reports a streak shorter than the one he can plainly see.

Given an integer array `values`, return the length of the longest run of consecutive equal elements. A single element is a run of length 1.

The editor starts with that code. Find the bug and fix it.

### Example 1

```
values = [3, 3, 1, 4, 4, 4]
Output: 3
```

The three 4s at the end form the longest run.

### Example 2

```
values = [2, 5, 2, 8, 8, 8, 7]
Output: 3
```

### Example 3

```
values = [1, 2, 1, 2, 2, 1, 1]
Output: 2
```

### Constraints

- 1 ≤ values.length ≤ 10⁵
- −10⁹ ≤ values[i] ≤ 10⁹
