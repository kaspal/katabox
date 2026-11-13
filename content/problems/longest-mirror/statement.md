A sign painter likes words that read the same backwards, like `level` or `noon`. Given a line of lowercase text `text`, find the longest **contiguous** piece of it that reads the same backwards.

Return that piece. If several pieces have the greatest length, return the one that starts first.

### Example 1

```
text = "levels"
Output: "level"
```

### Example 2

```
text = "noonmadam"
Output: "madam"
```

### Example 3

```
text = "xy"
Output: "x"
```

Every single letter reads the same backwards; `x` comes first.

### Constraints

- 1 ≤ text.length ≤ 3000
- `text` has lowercase English letters only.
