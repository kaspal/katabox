Kenji is designing an escape room with `n` rooms numbered from 0 to n - 1. He wrote a function that tells him how many rooms a player can get to from the starting room, but it keeps promising more rooms than his floor plan actually has.

Each door `[a, b]` in `doors` joins rooms `a` and `b` and can be walked through in both directions; the same pair of rooms may appear more than once. Return the number of rooms reachable from room `start`, counting `start` itself.

The editor starts with that code. Find the bug and fix it.

### Example 1

```
n = 5, doors = [[0, 1], [1, 2], [3, 4]], start = 1
Output: 3
```

From room 1 the player can reach rooms 0, 1 and 2.

### Example 2

```
n = 4, doors = [[0,1],[0,1],[3,1],[3,0]], start = 0
Output: 3
```

Room 2 has no doors.

### Example 3

```
n = 4, doors = [[0,1],[1,2]], start = 2
Output: 3
```

### Constraints

- 1 ≤ n ≤ 10⁴
- 0 ≤ doors.length ≤ 2·10⁴
- doors[i] = [a, b] with 0 ≤ a, b < n and a ≠ b
- 0 ≤ start < n
