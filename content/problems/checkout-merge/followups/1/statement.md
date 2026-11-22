This time the supermarket wants **one ticket of each number** in the merged line. As
before, each line in `lines` is a linked list of ticket numbers, from the front of the
line to the back, already in non-decreasing order, and the same number may appear in
several lines or several times in one line.

Merge every line into a single line in **strictly increasing** order, keeping each ticket
number exactly once, and return it. Lines may be empty; if there are no customers at all,
return an empty line.

Lists are written as their values in order, so `[2, 5, 9]` is the list 2 → 5 → 9, and
`[]` is the empty list.

### Example 1

```
lines = [[1, 4, 4, 9], [2, 4, 9], [1, 3]]
Output: [1, 2, 3, 4, 9]
```

Ticket 4 appears three times and 1 and 9 twice each; each is kept once.

### Example 2

```
lines = [[2, 2], [1, 2, 7, 7, 8], [1]]
Output: [1, 2, 7, 8]
```

### Example 3

```
lines = [[2, 5, 6], [4, 7], [1, 3, 9]]
Output: [1, 2, 3, 4, 5, 6, 7, 9]
```

### Constraints

- 0 ≤ lines.length ≤ 10⁴
- 0 ≤ total number of customers ≤ 10⁵
- 1 ≤ ticket number ≤ 10⁶
- Each line is in non-decreasing order.
