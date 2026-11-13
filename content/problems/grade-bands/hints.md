Each score is handled on its own: build the answer one score at a time.
---
Check the bands from the top: is the score at least 90? Otherwise at least 80? And so on.
---
An `if / else if` chain checking `>= 90`, `>= 80`, `>= 70`, `>= 60`, with `"F"` as the final `else`, gives each band exactly once.
