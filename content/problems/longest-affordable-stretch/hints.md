Costs are never negative. If a stretch is over budget, what happens when you make it longer?
---
It stays over budget. So for each end stage, the best start only moves to the right as the end moves right.
---
Keep a window `[left, right]` and its total. Add `costs[right]`; while the total is over `budget`, subtract `costs[left]` and move `left` right. The window is then valid: compare its length with the best so far.
