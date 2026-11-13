## Approach: a difference array

Record each repair at its two ends: `change[a] += crews`, and `change[b + 1] −= crews` (skip it when `b + 1 = length`). Then sweep from left to right with a running total; at section `i` the running total is the sum of the crews of the repairs that have started and not yet ended, which is exactly that section's count.

### Walkthrough

`length = 5`, repairs `[0, 2, 1]`, `[1, 4, 2]`, `[3, 3, 5]` give `change = [1, 2, 0, 4, −5]` (the −1 at 3 and the +5 at 3 combine). The running totals are `[1, 3, 3, 7, 2]`.

**Complexity:** O(length + repairs) time, O(length) space. Adding every repair section by section is O(length·repairs).
