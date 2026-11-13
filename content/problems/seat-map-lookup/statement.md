A stadium prints a ticket number on every seat. In its seat map `seats`, numbers increase strictly from left to right along every row and strictly from top to bottom down every column (but a row does not have to start after the previous row ends).

Return whether some seat has the number `ticket`.

### Example 1

```
seats = [[2, 5, 9],
         [4, 8, 12],
         [7, 11, 15]],
ticket = 11
Output: true
```

### Example 2

```
seats = [[2, 5, 9], [4, 8, 12], [7, 11, 15]], ticket = 6
Output: false
```

### Example 3

```
seats = [[10]], ticket = 10
Output: true
```

### Constraints

- 1 ≤ rows, columns ≤ 300 (all rows have the same length)
- 0 ≤ seats[r][c], ticket ≤ 10⁹
- Rows and columns are strictly increasing.

A full scan also passes the tests; the exercise is a search in O(rows · log columns) or O(rows + columns).
