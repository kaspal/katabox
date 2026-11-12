Checking every run means about n²/2 runs. Instead, keep a running score: +1 for a win, −1 for a loss, 0 for a draw. What does the score look like at the two ends of a balanced run?
---
A run from match `i + 1` to match `j` is balanced exactly when the running score after match `j` equals the running score after match `i` (with the score before the first match being 0).
---
So count pairs of equal running scores. Walk through the season keeping a hash map of how many times each score has appeared; each new position adds the number of earlier positions with the same score.
