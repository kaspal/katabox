Checking every part with a stack means O(n²) parts, each checked in up to O(n) steps. A balanced part can never contain a closing bracket that finds no partner inside it. What do such brackets do to the text?
---
Run the usual stack, but push **positions** instead of brackets. When a closing bracket matches the one at the top, the part that just closed starts right after whatever position is now on top.
---
Start the stack with `-1`, a wall before the text. Push the position of every opening bracket. For a closing bracket that matches the bracket at the top position, pop and measure `i − top`. For anything else (nothing to match, or the wrong kind), position `i` becomes the new wall: reset the stack to just `i`.
