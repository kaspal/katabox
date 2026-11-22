This time the ranger wants to fence each pond. Ponds are found as before: frozen `#` cells that touch on a side belong to the same pond. A pond's **fence length** is the number of sides of its cells that do not touch another frozen cell: sides facing open water or ground (`.`), or lying on the map's edge. Two cells of the same pond that touch share no fence.

Return the fence length of every pond, from **largest to smallest** (equal lengths simply appear one after the other). Return an empty list if nothing is frozen.

### Example 1

```
ice = ["##..#", "#...#", "..#..", ".##.."]
Output: [8, 8, 6]
```

The two L-shaped ponds of 3 cells need 8 sides each, and the pond of 2 cells on the right edge needs 6.

### Example 2

```
ice = ["..##.", "..#..", "....#", "..#.#"]
Output: [8, 6, 4]
```

### Example 3

```
ice = ["#.#..", "###..", ".....", "....."]
Output: [12]
```

One pond of 5 cells: 20 sides, minus 2 for each of the 4 places where two of its cells touch.

### Constraints

- 1 ≤ ice.length ≤ 400
- 1 ≤ ice[i].length ≤ 400, all rows the same length
- Every character is `#` or `.`
