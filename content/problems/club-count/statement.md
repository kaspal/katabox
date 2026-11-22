Lucía organises the welcome week at her university and tracks how the new students cluster into friend groups. Her function reports the number of groups after each new friendship, but by Thursday it was claiming more merges than there could possibly have been.

There are `n` people numbered 0 to `n - 1`, and each starts in a club of their own. The entries of `friendships` arrive in order; each `[a, b]` means `a` and `b` become friends, and their two clubs become one (a friend of a friend is in the same club). Return a list with one number per friendship: the number of clubs right after that friendship is added. A friendship between two people already in the same club changes nothing.

The editor starts with that code. Find the bug and fix it.

### Example 1

```
n = 5, friendships = [[0, 1], [2, 1], [0, 3], [2, 3], [4, 0]]
Output: [4, 3, 2, 2, 1]
```

After `[2, 1]`, people 0, 1 and 2 share a club. `[2, 3]` joins two people who are already together through 0, so the count stays at 2.

### Example 2

```
n = 4, friendships = [[0,1],[0,1],[3,1],[3,0],[0,1]]
Output: [3,3,2,2,2]
```

### Example 3

```
n = 4, friendships = [[0,1],[1,2],[2,1]]
Output: [3,2,2]
```

### Constraints

- 2 ≤ n ≤ 10⁴
- 1 ≤ friendships.length ≤ 2·10⁴
- Each friendship is `[a, b]` with 0 ≤ a, b < n and a ≠ b
