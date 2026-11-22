Dmitri added a bracket check to his little code editor so it can warn about unbalanced snippets. It catches brackets that close in the wrong order, but some broken snippets still get a clean bill of health.

Given a string `code` made only of the characters `()[]{}`, return `true` if its brackets are balanced and `false` otherwise. A string is balanced when every closing bracket matches the most recent unmatched opening bracket of the same kind, and no opening bracket is left unmatched at the end.

The editor starts with that code. Find the bug and fix it.

### Example 1

```
code = "{[()]}"
Output: true
```

### Example 2

```
code = "{()}[]"
Output: true
```

### Example 3

```
code = "()({[)"
Output: false
```

The `)` at the end tries to close the `[`, which is the most recent unmatched opening bracket.

### Constraints

- 1 ≤ code.length ≤ 10⁵
- code contains only the characters `()[]{}`.
