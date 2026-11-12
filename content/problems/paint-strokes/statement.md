Volunteers repaint a very long fence, measured in centimetres from its left end. Each entry `[start, end]` of `strokes` says someone painted the stretch from `start` up to, but not including, `end`. Strokes overlap freely; a centimetre painted twice still counts once.

Return the total length of fence that has been painted.

### Example 1

```
strokes = [[2, 6], [10, 12], [4, 8]]
Output: 8
```

[2, 6] and [4, 8] together cover 2 to 8 (6 cm); [10, 12] adds 2 cm.

### Example 2

```
strokes = [[0, 5], [5, 9]]
Output: 9
```

The strokes meet at 5 without overlapping.

### Example 3

```
strokes = [[3, 30], [7, 9], [3, 4]]
Output: 27
```

### Constraints

- 1 ≤ strokes.length ≤ 10⁵
- strokes[i] = [start, end] with 0 ≤ start < end ≤ 10⁹
