Shelf labels in a warehouse are too narrow for long codes, so codes are shortened. Split `text` into **runs**: maximal groups of equal consecutive letters. Write each run as its letter, followed by the run's length **only if the length is greater than 1**.

Return the shortened label.

### Example 1

```
text = "aaabcc"
Output: "a3bc2"
```

The runs are `aaa`, `b` and `cc`.

### Example 2

```
text = "abab"
Output: "abab"
```

### Example 3

```
text = "zzzzzzzzzzzz"
Output: "z12"
```

### Constraints

- 1 ≤ text.length ≤ 10⁵
- `text` contains only lowercase English letters.
