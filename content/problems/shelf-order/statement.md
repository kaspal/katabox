A storeroom labels its boxes with lowercase letters and digits, like `box2` or `shelf10b`. Sorting the labels letter by letter puts `box10` before `box2`, which confuses everyone, so the storeroom wants **natural order**:

- Split each label into **chunks**: maximal runs of letters and maximal runs of digits (`shelf10b` → `shelf`, `10`, `b`).
- Compare two labels chunk by chunk, from the left, until two chunks differ:
  - two digit chunks compare by their **number value** (`2` < `10`; `007` equals `7`);
  - two letter chunks compare **alphabetically**;
  - a digit chunk comes before a letter chunk.
- If one label runs out of chunks first while all chunks so far were equal, it comes first.
- Labels that are equal under these rules (like `a01` and `a1`) keep their original order.

Return the labels in natural order.

### Example 1

```
labels = ["box10", "box2", "box1", "apple", "box2a"]
Output: ["apple", "box1", "box2", "box2a", "box10"]
```

### Example 2

```
labels = ["a01", "a1", "a001"]
Output: ["a01", "a1", "a001"]
```

All three are equal in natural order, so they stay as given.

### Example 3

```
labels = ["9", "10", "x", "1x"]
Output: ["1x", "9", "10", "x"]
```

### Constraints

- 1 ≤ labels.length ≤ 10⁴
- 1 ≤ labels[i].length ≤ 20, only lowercase English letters and digits
