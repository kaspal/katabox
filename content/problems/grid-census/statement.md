A forest ranger flies a drone over a replanted hillside and turns the photo into a map of square cells: `#` is a young tree, `.` is grass. Back at the station, the planting crews call in with questions such as "how many trees are on row 12?" or "how many on column 40?", and the ranger wants to answer each one at once.

This is a **program problem**: read the map and the questions from standard input and print the answers to standard output.

## Input format

- The first line holds `R C`, the number of rows and columns of the map.
- The next `R` lines are the map rows, top to bottom: each has exactly `C` characters, every one `#` or `.`, with no spaces.
- The next line holds `Q`, the number of questions.
- Each of the next `Q` lines is a question: `row r` or `col c`, with 0-based indexes (row 0 is the top row, column 0 the leftmost).

## Output format

Print `Q` lines, one per question in order: the number of trees in that row or column.

### Example 1

Input:

```
3 4
#..#
....
.###
4
row 0
row 1
col 3
col 0
```

Output:

```
2
0
2
1
```

Column 3 is the rightmost one: `#`, `.`, `#`.

### Example 2

Input:

```
1 1
#
2
row 0
col 0
```

Output:

```
1
1
```

### Example 3

Input:

```
2 5
.....
#.#.#
3
col 4
row 1
col 1
```

Output:

```
1
3
0
```

### Constraints

- 1 ≤ R, C ≤ 300
- 1 ≤ Q ≤ 2·10⁴
- 0 ≤ r < R and 0 ≤ c < C
