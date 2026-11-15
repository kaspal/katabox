A land registry stores every plot as a polygon: the corners of its fence, in order around the plot, with whole-number coordinates in metres. A surveyor checking an old record needs two figures for each plot: its area, and how many whole-number points lie strictly inside it (the places where a marker post could stand without touching the fence).

This is a **program problem**: read the plot from standard input and print its area and its inside points to standard output.

## Input format

- The first line holds `n`, the number of corners.
- Then `n` lines, each `x y`: the corners in order around the plot, clockwise or anticlockwise. The fence runs from each corner to the next, and from the last corner back to the first.

## Output format

Print one line: the area with exactly one decimal (it always ends in `.0` or `.5`), a space, and the number of whole-number points strictly inside the plot. Points on the fence do not count.

### Example 1

Input:

```
4
0 0
4 0
4 3
0 3
```

Output:

```
12.0 6
```

A 4 × 3 rectangle: the inside points are `x` from 1 to 3 and `y` from 1 to 2, six in all.

### Example 2

Input:

```
3
0 0
5 0
0 3
```

Output:

```
7.5 4
```

The inside points are (1, 1), (2, 1), (3, 1) and (1, 2).

### Example 3

Input:

```
6
0 0
0 5
2 5
2 2
6 2
6 0
```

Output:

```
18.0 8
```

An L-shaped plot listed clockwise.

### Constraints

- 3 ≤ n ≤ 1000
- -10⁶ ≤ x, y ≤ 10⁶
- The corners are different points.
- The plot is a simple polygon: edges meet only at the corner shared by neighbouring edges, and an edge never folds back over the previous one. A corner may lie on a straight line between its neighbours.
- The area is positive.
