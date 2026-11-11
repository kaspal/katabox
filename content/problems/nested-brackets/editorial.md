## Approach: a stack of open brackets

Scan `text` once and skip letters. Push every opening bracket. For a closing bracket, the innermost open bracket is the top of the stack: if the stack is empty or the top is of another kind, the text is not nested properly; otherwise pop it. When the scan ends, any bracket left on the stack was never closed.

### Walkthrough

`text = "({)}"`:

| char | stack before | action |
| --- | --- | --- |
| `(` | [] | push |
| `{` | [`(`] | push |
| `)` | [`(`, `{`] | top is `{`, not `(`: return false |

**Complexity:** O(n) time, O(n) extra space for the stack. Repeatedly deleting adjacent pairs such as `()` until nothing changes is also correct, but deep nesting makes it O(n²).
