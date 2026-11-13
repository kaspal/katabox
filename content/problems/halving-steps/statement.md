A puzzle game shows a number `n` and one rule per step:

- if the number is **even**, halve it;
- if it is **odd** (and not 1), multiply it by 3 and add 1.

The game ends when the number reaches 1. Return how many steps it takes to reach 1 starting from `n`.

### Example 1

```
n = 6
Output: 8
```

6 → 3 → 10 → 5 → 16 → 8 → 4 → 2 → 1: eight steps.

### Example 2

```
n = 1
Output: 0
```

### Example 3

```
n = 16
Output: 4
```

### Constraints

- 1 ≤ n ≤ 10⁶
- Every n in this range reaches 1, and the numbers on the way stay below 10¹¹.
