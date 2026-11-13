A meeting room's booking sheet lists time ranges, `bookings[i] = [start, end]`, both moments included. To print the room's busy hours, overlapping bookings must be combined: two ranges that share at least one moment (also just an end point) become one range covering both.

Return the combined ranges, sorted by start.

### Example 1

```
bookings = [[4, 7], [1, 2], [6, 9], [12, 14]]
Output: [[1, 2], [4, 9], [12, 14]]
```

### Example 2

```
bookings = [[5, 5], [5, 8], [9, 10]]
Output: [[5, 8], [9, 10]]
```

[5, 8] and [9, 10] share no moment, so they stay apart.

### Example 3

```
bookings = [[3, 20], [4, 6]]
Output: [[3, 20]]
```

### Constraints

- 1 ≤ bookings.length ≤ 10⁴
- 0 ≤ start ≤ end ≤ 10⁶
