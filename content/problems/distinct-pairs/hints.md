Lists without repeated prices are handled correctly. Try one where a matching price appears twice on each side, such as `prices = [1, 1, 3, 3], budget = 4`.
---
Moving the pointers when the sum is too small or too large is fine. Look at what happens right after a match is counted.
---
After a match, the code moves each pointer by only one position, so the next step can land on the same two values and count them again. Instead, move `i` past every copy of `prices[i]` and `j` past every copy of `prices[j]` (keeping `i < j`) before continuing.
