A coworking space receives a list of `meetings`, each `[start, end]` in minutes. A meeting uses its room from `start` up to, but not including, `end`: a meeting ending at minute 30 and another starting at minute 30 can use the same room. Two meetings whose times overlap need different rooms.

Return the smallest number of rooms that can host every meeting.

### Example 1

```
meetings = [[0, 40], [5, 15], [20, 35]]
Output: 2
```

The first meeting overlaps both others, but those two do not overlap each other.

### Example 2

```
meetings = [[10, 20], [20, 30], [30, 40]]
Output: 1
```

### Example 3

```
meetings = [[1, 5], [2, 6], [3, 7], [6, 8]]
Output: 3
```

### Constraints

- 1 ≤ meetings.length ≤ 2·10⁴
- meetings[i] = [start, end] with 0 ≤ start < end ≤ 10⁹
