This time the bakery logs its daily **profit**, `profits[i]`, and a bad day can be a loss
(a negative number). For the poster it may pick a run of **at least 1 and at most `k`**
consecutive days, whichever looks best.

Return the largest total profit of any run of 1 to `k` consecutive days. A run always has
at least one day, so if every day is a loss, the answer is the smallest loss.

### Example 1

```
profits = [3, -5, 4, 2, -1, 6], k = 3
Output: 7
```

Days 3 to 5 give 2 − 1 + 6 = 7. Days 2 to 5 would give 11, but that is four days.

### Example 2

```
profits = [-4, 2, -3, 9, 8, 9], k = 2
Output: 17
```

### Example 3

```
profits = [-12, -3, -9, -5, -6, -8], k = 1
Output: -3
```

### Constraints

- 1 ≤ k ≤ profits.length ≤ 2·10⁵
- −10⁴ ≤ profits[i] ≤ 10⁴
