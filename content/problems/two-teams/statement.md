A five-a-side tournament has `n` players, numbered 0 to n − 1, who must be split into two teams. Some pairs of players are rivals, `rivalries[i] = [a, b]`, and rivals must play on **different** teams.

Return whether such a split exists. Players without rivals can join either team, and a team may be empty.

### Example 1

```
n = 4, rivalries = [[0, 1], [1, 2], [2, 3], [3, 0]]
Output: true
```

Players 0 and 2 against 1 and 3.

### Example 2

```
n = 3, rivalries = [[0, 1], [1, 2], [2, 0]]
Output: false
```

Three players who are all rivals cannot fit in two teams.

### Example 3

```
n = 5, rivalries = []
Output: true
```

### Constraints

- 1 ≤ n ≤ 10⁵
- 0 ≤ rivalries.length ≤ 2·10⁵
- 0 ≤ a, b < n, a ≠ b, and no pair is listed twice.
