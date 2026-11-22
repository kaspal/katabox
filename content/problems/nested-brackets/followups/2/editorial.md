## Idea

A closing bracket with nothing to match, or with the wrong kind on top, splits the text: no balanced part can cross it. Keep a stack of positions with the latest such wall at the bottom (`-1` at the start) and, above it, the opening brackets still waiting. When `text[i]` closes the bracket at the top position, pop it: everything after the new top, up to `i`, is now balanced, so its length is `i − top`; keep the largest. On a mismatch, `i` becomes the new wall: clear the stack and leave `i` alone at the bottom. An opening bracket that never closes simply stays on the stack and acts as a wall for the parts after it.

## Complexity

O(n) time, since each position is pushed and popped at most once, and O(n) extra memory for the stack.
