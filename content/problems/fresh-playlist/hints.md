Growing a stretch from every starting play repeats almost all the work when stretches are long. What happens to a fresh stretch when you add the next play?
---
Keep a window `[start, end]` that is always fresh. When the new play `x` was already played inside the window, the window must start just after that earlier play of `x`.
---
Remember the last position of every song (a hash map). For each `end`: if `last[x] ≥ start`, set `start = last[x] + 1`; then `last[x] = end`, and the window length is `end − start + 1`.
