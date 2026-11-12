An archipelago has `n` islands, numbered `0` to `n − 1`. Each entry `[a, b]` of `links` is a ferry line that sails both ways between islands `a` and `b`. Two islands belong to the same **group** when you can get from one to the other by ferry, possibly changing boats on other islands.

Return the number of groups.

### Example 1

```
n = 6, links = [[0, 3], [4, 5], [3, 1]]
Output: 3
```

The groups are {0, 1, 3}, {2} and {4, 5}.

### Example 2

```
n = 3, links = []
Output: 3
```

With no ferries, every island is its own group.

### Example 3

```
n = 4, links = [[2, 1], [1, 2], [0, 3], [3, 2]]
Output: 1
```

### Constraints

- 1 ≤ n ≤ 10⁵
- 0 ≤ links.length ≤ 10⁵
- links[i] = [a, b] with 0 ≤ a, b < n and a ≠ b (the same pair may appear more than once)
