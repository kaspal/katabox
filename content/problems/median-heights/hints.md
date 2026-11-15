Read the whole input at once and split it into tokens. Read `T`, then for each section read `n` and take the next `n` tokens as its heights.
---
Once a section's heights are sorted, the median is simply the one at a fixed position. Which position is it for `n = 5`? For `n = 4`, where you want the lower of the two middle ones?
---
Sort the heights as numbers and print `heights[(n − 1) / 2]` (0-based, integer division): position 2 for `n = 5`, position 1 for `n = 4`, position 0 for `n = 1`. Collect the answers and print them once at the end.
