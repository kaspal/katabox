This time, instead of counting rooms, give every meeting a **room number**. Rooms are
numbered 1, 2, 3, … and the coworking space hands them out like this:

- Meetings are taken in order of start time; meetings with the same start are taken in
  their order in the input.
- Each meeting gets the **smallest room number that is free** at its start. A room is
  free if every meeting already in it has ended by then: as before, a room freed at
  minute `t` can host a meeting starting at minute `t`.

Return `rooms`, where `rooms[i]` is the room number given to `meetings[i]`.

### Example 1

```
meetings = [[0, 30], [5, 10], [15, 20], [10, 40]]
Output: [1, 2, 3, 2]
```

[10, 40] starts as [5, 10] ends, so it reuses room 2; at minute 15 rooms 1 and 2 are
both busy, so [15, 20] opens room 3.

### Example 2

```
meetings = [[8, 27], [54, 60], [48, 51], [16, 20], [31, 56]]
Output: [1, 2, 2, 2, 1]
```

### Example 3

```
meetings = [[55, 60], [3, 6], [5, 17], [53, 59], [47, 60]]
Output: [3, 1, 2, 2, 1]
```

### Constraints

- 1 ≤ meetings.length ≤ 2·10⁴
- meetings[i] = [start, end] with 0 ≤ start < end ≤ 10⁹
