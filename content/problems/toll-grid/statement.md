A delivery van crosses a city laid out as a grid of districts. Entering district `(r, c)` costs a toll of `tolls[r][c]` coins, and this includes the starting district, top-left `(0, 0)`, and the destination, bottom-right. From a district the van can move up, down, left or right to a neighbouring one, and pays again every time it enters a district.

The driver also has **one coupon**: in a single district of their choice, they pay half its toll, rounded down. The coupon can be used at most once (also on the start or the destination).

Return the fewest coins needed to get from the top-left to the bottom-right district.

### Example 1

```
tolls = [[1, 9, 1], [1, 9, 1], [1, 1, 1]]
Output: 4
```

Down the left column and along the bottom row: five districts of toll 1 cost 5, and the coupon makes one of them cost 0.

### Example 2

```
tolls = [[7]]
Output: 3
```

The start is also the destination; with the coupon its toll of 7 costs 3.

### Example 3

```
tolls = [[1, 100, 1], [50, 100, 50], [1, 100, 1]]
Output: 103
```

Every route crosses the middle column. Going through `(0, 1)` costs 1 + 100 + 1 + 50 + 1 = 153, and the coupon on the 100 saves 50.

### Constraints

- 1 ≤ rows, cols ≤ 150, all rows of `tolls` have the same length
- 1 ≤ tolls[r][c] ≤ 10⁴
