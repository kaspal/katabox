Checking every run is O(n²). Keep a running score: +1 for a win, −1 for a loss, 0 for a
draw. What does a balanced run look like in terms of that score?
---
A run is balanced exactly when the score after its last match equals the score just
before its first match. For the longest run ending at match `i`, which earlier moment
with the same score do you want?
---
Remember the **first** position where each score appeared, starting with score 0 at
position −1. At each match, if the current score was seen before, the run since that
first time is balanced; keep the longest. Never overwrite a first position.
