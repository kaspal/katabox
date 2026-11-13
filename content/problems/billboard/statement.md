A fairground fence is made of side-by-side panels, each 1 metre wide; `panels[i]` is the height of panel `i`, measured from the ground. An advertiser wants to paste one rectangular poster on the fence: its bottom edge is on the ground, it covers whole panels, and it may not stick out above any panel it covers.

Return the largest possible poster area.

### Example 1

```
panels = [3, 1, 4, 4, 2, 5]
Output: 8
```

Either 2 panels of height 4 (panels 2-3), or 4 panels of height 2 (panels 2-5).

### Example 2

```
panels = [6]
Output: 6
```

### Example 3

```
panels = [2, 2, 2, 0, 3]
Output: 6
```

### Constraints

- 1 ≤ panels.length ≤ 10⁵
- 0 ≤ panels[i] ≤ 10⁴
