An engineer is reclaiming land from a shallow sea, mapped as a grid of `R` rows and `C` columns of squares. All of it starts as water. Each day her crew fills one square with earth, and the planners want to know how many islands there are after every fill. Two land squares belong to the same island when they share a side; touching only at a corner does not join them. The crew sometimes fills a square that is already land, which changes nothing.

This is a **program problem**: read the map size and the filled squares from standard input and print the number of islands after each one to standard output.

## Input format

- The first line holds `R C Q`: the number of rows, the number of columns, and the number of filled squares.
- Then `Q` lines, each `r c`: the row and column of the square filled that day, in order.

## Output format

Print `Q` lines: after each filled square, in order, the number of islands on the map.

### Example 1

Input:

```
3 3 5
1 1
1 3
2 2
1 2
3 3
```

Output:

```
1
2
3
1
2
```

The squares (1, 1), (1, 3) and (2, 2) touch only at corners: three islands. Filling (1, 2) joins all three into one. (3, 3) touches (2, 2) only at a corner, so it starts a new island.

### Example 2

Input:

```
1 1 2
1 1
1 1
```

Output:

```
1
1
```

The same square is filled twice: the second fill changes nothing.

### Example 3

Input:

```
2 4 4
1 1
2 4
1 4
2 1
```

Output:

```
1
2
2
2
```

### Constraints

- 1 ≤ R, C ≤ 1000
- 1 ≤ Q ≤ 5·10⁴
- 1 ≤ r ≤ R
- 1 ≤ c ≤ C
