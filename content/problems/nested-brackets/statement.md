A configuration linter reads `text`, a line made of lowercase letters and the brackets `(`, `)`, `[`, `]`, `{` and `}`. Letters are ignored.

Return `true` when the brackets are properly nested: every opening bracket is closed later by a bracket of the same kind, and a pair that opens inside another pair also closes inside it. Otherwise return `false`.

### Example 1

```
text = "f(a[b]{c})"
Output: true
```

### Example 2

```
text = "(x]"
Output: false
```

The `(` is closed by a `]`, a different kind.

### Example 3

```
text = "({)}"
Output: false
```

The `{` opens inside the `(` pair but closes after it.

### Constraints

- 1 ≤ text.length ≤ 10⁵
- `text` contains only lowercase English letters and `()[]{}`.
