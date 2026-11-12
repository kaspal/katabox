A festival prints messages on long ribbons that people read from either end. A ribbon is a **mirror ribbon** when its letters and digits read the same from left to right as from right to left. Spaces, punctuation and every other symbol are ignored, and so is the difference between upper and lower case.

Given the ribbon's `text`, return `true` if it is a mirror ribbon and `false` otherwise. A ribbon with no letters or digits at all is a mirror ribbon.

### Example 1

```
text = "Stack cats, 2 by 2!"
Output: false
```

Keeping only letters and digits in lower case gives `stackcats2by2`, which reversed is `2yb2stackcats`.

### Example 2

```
text = "Top spot: 7 -- 7 tops pot"
Output: true
```

The letters and digits are `topspot77topspot`, the same in both directions.

### Example 3

```
text = "...!"
Output: true
```

### Constraints

- 1 ≤ text.length ≤ 10⁵
- text contains printable ASCII characters (codes 32 to 126).
