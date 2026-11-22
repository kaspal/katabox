Try the tiniest case with an answer that does not use the lightest jar: `weights = [1, 2]`, `k = 1`, `target = 2`. Then print `pick` at the start of every call to `go`.
---
The stopping test and the running total are fine. Look at what happens to `pick` inside the `for` loop after the recursive call returns: is it the same list it was before the choice was made?
---
Every choice is pushed onto `pick` but never taken back, so the second option of the loop is tried on top of the first, and `pick` soon grows past `k` and never matches again. Undo the choice right after the recursive call: `pick.pop()`.
