The checkout lines are the same: each line in `lines` is a linked list of ticket numbers,
from the front of the line to the back, in non-decreasing order. This time every line has
at least one customer.

Before closing, the manager wants a **fair sample** for a survey: a range of ticket
numbers `[lo, hi]` that contains at least one customer's ticket from **every** line. Return
the narrowest such range as `[lo, hi]`: the smallest `hi − lo`, and among ranges of that
width, the one with the smallest `lo`.

Lists are written as their values in order, so `[2, 5, 9]` is the list 2 → 5 → 9.

### Example 1

```
lines = [[3, 8, 20], [5, 9, 14], [1, 10, 25]]
Output: [8, 10]
```

Tickets 8, 9 and 10 come from the three lines; no narrower range reaches all three.

### Example 2

```
lines = [[2, 2, 7], [7]]
Output: [7, 7]
```

### Example 3

```
lines = [[6, 9]]
Output: [6, 6]
```

### Constraints

- 1 ≤ lines.length ≤ 10⁴
- 1 ≤ total number of customers ≤ 10⁵, and every line has at least one
- 1 ≤ ticket number ≤ 10⁶
- Each line is in non-decreasing order.
