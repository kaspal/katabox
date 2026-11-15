A building's wall is drawn as a grid of squares: `.` is a free square and `#` is a blocked one (a window, a pipe, a sign). The owners want to hang one rectangular banner, aligned to the grid, that covers only free squares. How large can it be?

This is a **program problem**: read the wall from standard input and print the largest area to standard output.

## Input format

- The first line holds `R C`: the number of rows and columns of the wall.
- Then `R` lines, each with exactly `C` characters, `.` or `#`, from the top row down.

## Output format

Print one number: the largest number of squares a banner can cover, or `0` if no square is free.

### Example 1

Input:

```
4 5
.#...
.....
##...
.#..#
```

Output:

```
9
```

The last three columns of the top three rows are all free: a 3 × 3 banner.

### Example 2

Input:

```
2 3
###
###
```

Output:

```
0
```

Every square is blocked.

### Example 3

Input:

```
3 4
....
.#..
....
```

Output:

```
6
```

The last two columns are free from top to bottom: 3 × 2 = 6, more than any full row (4).

### Constraints

- 1 ≤ R, C ≤ 1500
- Each row has exactly `C` characters, `.` or `#`
