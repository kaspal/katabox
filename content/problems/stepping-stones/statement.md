A path of stepping stones crosses a river: stone 0 is on one bank and stone n − 1 on the other. From stone `i`, a hiker can jump forward to any stone at most `jumps[i]` stones ahead (a value of 0 means they cannot move from there).

Starting on stone 0, return whether the hiker can reach the last stone.

### Example 1

```
jumps = [2, 1, 0, 3, 1]
Output: false
```

Stones 1 and 2 are reachable, but neither gets past stone 2, where the hiker cannot move.

### Example 2

```
jumps = [2, 0, 3, 0, 0, 1]
Output: true
```

0 → 2 → 5.

### Example 3

```
jumps = [0]
Output: true
```

### Constraints

- 1 ≤ jumps.length ≤ 10⁵
- 0 ≤ jumps[i] ≤ 10⁵
