This time the linter only looks at round brackets: `text` holds lowercase letters (still ignored), `(` and `)`. Instead of saying whether the text is balanced, return the **fewest brackets you must insert**, anywhere in `text`, so that its brackets become properly nested: every `(` is closed by a later `)`, and every `)` closes an earlier `(`.

You may insert `(` or `)` at any position, but you cannot delete anything. A text that is already balanced needs 0 insertions.

### Example 1

```
text = "())(a("
Output: 3
```

The second `)` needs a `(` inserted before it, and each of the last two `(` needs a `)` after it.

### Example 2

```
text = "()())"
Output: 1
```

### Example 3

```
text = "a)aa"
Output: 1
```

One `(` before the `)` is enough.

### Constraints

- 1 ≤ text.length ≤ 10⁵
- `text` contains only lowercase English letters, `(` and `)`.
