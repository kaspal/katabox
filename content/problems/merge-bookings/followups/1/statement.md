This time there are **two** meeting rooms, each with its own booking sheet: `roomA` and
`roomB`. Each booking is `[start, end]`, both moments included, and bookings on the same
sheet may overlap. Find the moments when **both rooms are busy at once**.

Return those moments as combined ranges `[start, end]` (both ends included), sorted by
start. As before, ranges that share a moment are one range, and ranges that share no
moment stay apart, even when one ends right before the next begins. Return an empty list
if the rooms are never busy at the same moment.

### Example 1

```
roomA = [[1, 4], [8, 12], [3, 6]], roomB = [[5, 9], [11, 11]]
Output: [[5, 6], [8, 9], [11, 11]]
```

Room A is busy during [1, 6] and [8, 12]; room B during [5, 9] and [11, 11].

### Example 2

```
roomA = [[4, 8], [2, 4], [3, 6]], roomB = [[15, 20], [12, 13], [3, 6]]
Output: [[3, 6]]
```

### Example 3

```
roomA = [[1, 1], [2, 4], [5, 10]], roomB = [[9, 11], [19, 20], [19, 19], [18, 20]]
Output: [[9, 10]]
```

### Constraints

- 1 ≤ roomA.length, roomB.length ≤ 10⁴
- each booking is [start, end] with 0 ≤ start ≤ end ≤ 10⁶
