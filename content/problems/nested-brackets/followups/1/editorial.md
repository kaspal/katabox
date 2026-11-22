## Idea

In the base problem one bad bracket made the answer `false`; here every bad bracket gets its own repair. Scan left to right with `open`, the number of `(` not yet closed. A `)` closes one of them when `open > 0`; otherwise no bracket to its left can ever match it, so one `(` has to be inserted (right before it works). At the end, each of the `open` brackets still waiting needs its own `)`. Every insertion fixes exactly one unmatched bracket and every unmatched bracket needs one, so `added + open` is the minimum.

## Complexity

O(n) time and O(1) extra memory: with only one kind of bracket, a counter replaces the stack.
