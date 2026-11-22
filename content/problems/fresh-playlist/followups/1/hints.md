If a stretch follows the rule, every shorter stretch inside it does too. That is what made a sliding window work in the base problem.
---
Keep a window `[start, end]` and a count of each song inside it. Extending the window by one play can only break the rule for the song just added.
---
Add `plays[end]` to its count; while that count is above 2, remove `plays[start]` from the counts and move `start` right. The window is then valid; keep the largest length you see.
