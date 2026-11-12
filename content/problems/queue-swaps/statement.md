For the class photo, the children stand in one row and must end up ordered from shortest to tallest (children of the same height may stand in either order). The photographer only allows one kind of move: two children standing **next to each other** swap places.

`heights` lists the children's heights from left to right. Return the **fewest swaps** needed.

### Example 1

```
heights = [3, 1, 2]
Output: 2
```

Swap 3 with 1, then 3 with 2: `[1, 3, 2]`, then `[1, 2, 3]`.

### Example 2

```
heights = [1, 2, 2, 3]
Output: 0
```

### Example 3

```
heights = [4, 3, 2, 1]
Output: 6
```

### Constraints

- 1 ≤ heights.length ≤ 10⁵
- 1 ≤ heights[i] ≤ 10⁹
