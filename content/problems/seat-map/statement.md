Amara runs the box office of a small theatre and wrote a function that draws the seating plan after the night's bookings. On her first test, with a single row, it worked perfectly. Then a customer booked one seat in a bigger hall and the plan showed seats taken that nobody had booked.

The hall has `rows` rows and `cols` seats per row, and every seat starts free. Each entry of `bookings` is `[row, col]` and marks that seat as taken; a seat can be booked more than once and simply stays taken. Return the plan as a list of `rows` strings, one per row from row 0, each of length `cols`, where `.` is a free seat and `X` a taken one.

The editor starts with that code. Find the bug and fix it.

### Example 1

```
rows = 3, cols = 4, bookings = [[0, 1], [2, 3], [0, 1]]
Output: [".X..", "....", "...X"]
```

Seat `[0, 1]` is booked twice; it is still just taken.

### Example 2

```
rows = 2, cols = 3, bookings = [[0,1],[1,1]]
Output: [".X.",".X."]
```

### Example 3

```
rows = 2, cols = 3, bookings = [[1,0]]
Output: ["...","X.."]
```

### Constraints

- 1 ≤ rows, cols ≤ 300
- 0 ≤ bookings.length ≤ 10⁴
- Each booking is `[row, col]` with 0 ≤ row < rows and 0 ≤ col < cols
