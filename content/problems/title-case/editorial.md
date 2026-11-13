## Approach: remember the previous character

Scan the text once, keeping a flag `atStart` that is true at the beginning and after every space:

- a space is copied and sets `atStart = true`;
- any other character is uppercased if `atStart`, lowercased otherwise, and sets `atStart = false`.

Uppercasing or lowercasing a digit leaves it unchanged, so a word like `66` stays as it is, and `3RD` becomes `3rd`.

Splitting on spaces and joining again would lose repeated spaces, which the statement asks to keep.

**Complexity:** O(n) time, O(n) for the result.
