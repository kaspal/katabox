A child plays with rectangular rubber stamps on a sheet of squared paper with `rows` rows and `cols` columns of cells. Stamp `[top, left, bottom, right]` inks every cell from row `top` to row `bottom` and from column `left` to column `right`, all included. Cells can be inked many times.

Return `[most, cells]`: the largest number of times any cell was inked, and how many cells were inked exactly that many times.

### Example 1

```
rows = 3, cols = 4, stamps = [[0, 0, 1, 1], [1, 1, 2, 3], [0, 1, 2, 1]]
Output: [3, 1]
```

The ink counts are:

```
1 2 0 0
1 3 1 1
0 2 1 1
```

### Example 2

```
rows = 2, cols = 2, stamps = [[0, 0, 1, 1], [0, 0, 1, 1]]
Output: [2, 4]
```

### Example 3

```
rows = 1, cols = 5, stamps = [[0, 0, 0, 1], [0, 3, 0, 4]]
Output: [1, 4]
```

### Constraints

- 1 ≤ rows, cols ≤ 500
- 1 ≤ stamps.length ≤ 10⁵
- 0 ≤ top ≤ bottom < rows, 0 ≤ left ≤ right < cols
