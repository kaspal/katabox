A garden wall is built from stone columns of width 1 standing side by side; `columns[i]` is the height of column `i`. After a storm, water is caught in the dips between columns: above each column, water stands as high as the **lower** of the tallest column to its left and the tallest column to its right (water flows off both ends of the wall).

Return the total amount of trapped water, in unit squares.

### Example 1

```
columns = [3, 0, 2, 0, 4]
Output: 7
```

Above the columns of height 0, 2 and 0, the water reaches height 3: 3 + 1 + 3.

### Example 2

```
columns = [1, 2, 3]
Output: 0
```

### Example 3

```
columns = [5, 1, 1, 6, 0, 2]
Output: 10
```

4 + 4 above the two 1s, and 2 above the 0.

### Constraints

- 1 ≤ columns.length ≤ 2·10⁵
- 0 ≤ columns[i] ≤ 10⁴
