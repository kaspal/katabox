A tiler is covering a rectangular floor of `width` × `height` centimetres with identical **square** tiles. Tiles cannot be cut, and the floor must be covered exactly, edge to edge, with no gaps.

Return the side length, in whole centimetres, of the **largest** square tile that works.

### Example 1

```
width = 240, height = 180
Output: 60
```

Tiles of 60 cm fit 4 across and 3 down. A 120 cm tile would fit the width twice but not the height.

### Example 2

```
width = 35, height = 35
Output: 35
```

One tile covers the whole floor.

### Example 3

```
width = 64, height = 27
Output: 1
```

### Constraints

- 1 ≤ width, height ≤ 10¹²
