An app has 16 optional features, numbered 0 to 15. Each user's settings are stored as a number whose bit `f` is 1 when feature `f` is switched on: `users[i]` is user `i`'s number.

The product team asks questions like "how many users have **all** of these features on (and maybe others too)?". Each question is also a number, with bit `f` set for every feature it asks about. For each question in `queries`, return how many users have every asked feature on.

### Example 1

```
users = [5, 7, 2, 15], queries = [5, 2, 8, 0]
Output: [3, 3, 1, 4]
```

Question 5 (binary 0101) asks for features 0 and 2: users 5, 7 and 15 have both. Question 0 asks for nothing, so every user counts.

### Example 2

```
users = [0, 0], queries = [1]
Output: [0]
```

### Example 3

```
users = [65535], queries = [65535, 32768]
Output: [1, 1]
```

### Constraints

- 1 ≤ users.length, queries.length ≤ 10⁵
- 0 ≤ users[i], queries[j] < 2¹⁶
