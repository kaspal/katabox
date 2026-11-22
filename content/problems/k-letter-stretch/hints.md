Example 1 passes, but Example 2 does not. Try a tiny case too: `text = "aab", k = 1`. Which substring would give the length the code returns, and how many letters does it use?
---
Adding a letter, shrinking the window and updating `best` are all present. Look at the order they happen in inside the loop.
---
`best` is updated right after a letter is added, before the `while` loop has removed the extra letters, so a window with more than `k` distinct letters can be recorded. Move the `best = max(...)` line after the shrink loop.
