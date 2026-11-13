A fence is a row of upright boards, one metre apart; `boards[i]` is the height of board `i`. A farmer wants to build a rain-water tank by picking **two** boards and closing the space between them (the boards in between are taken out). The tank's width is the distance between the two boards, and the water can only rise to the height of the **lower** of the two.

Return the largest amount of water a tank can hold: `(j − i) · min(boards[i], boards[j])` for the best pair `i < j`.

### Example 1

```
boards = [2, 7, 4, 1, 8, 3, 6]
Output: 30
```

Boards 1 and 6: width 5, height min(7, 6) = 6.

### Example 2

```
boards = [5, 5]
Output: 5
```

### Example 3

```
boards = [0, 3, 0]
Output: 0
```

### Example 4

```
boards = [1, 9, 9, 1]
Output: 9
```

### Constraints

- 2 ≤ boards.length ≤ 2·10⁵
- 0 ≤ boards[i] ≤ 10⁴
