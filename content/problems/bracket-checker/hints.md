Wrong closing brackets are caught. What about a snippet that simply never closes, like `"("`?
---
The loop handles every character correctly. Look at what the function returns once the loop is over.
---
After the loop, the stack may still hold opening brackets that were never closed, but the code returns `True` regardless. Return `not stack` instead (in TypeScript, `stack.length === 0`).
