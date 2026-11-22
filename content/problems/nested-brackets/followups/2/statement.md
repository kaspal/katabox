This time `text` holds only brackets, `()[]{}`, with no letters, and as a whole it does not have to be properly nested. Find the longest **part** of it, a run of consecutive characters, that is properly nested on its own, with the same rule as before: every opening bracket is closed later by one of the same kind, and a pair that opens inside another pair also closes inside it.

Return the length of that part, or `0` if no part is balanced (an empty part does not count).

### Example 1

```
text = "](()[]){[}"
Output: 6
```

The part `(()[])`, from position 1 to 6.

### Example 2

```
text = "(){}[]["
Output: 6
```

The first six characters; the last `[` is never closed.

### Example 3

```
text = "(()))("
Output: 4
```

### Constraints

- 1 ≤ text.length ≤ 10⁵
- `text` contains only `()[]{}`.
