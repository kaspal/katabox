The outline can only change where some building starts or ends. How many such positions are there, and what do you need at each one?
---
Sweep the positions from left to right. At each one you need the tallest building that has started and not yet ended. Which structure gives the maximum while buildings come and go?
---
A max-heap of `(height, right)`. At position `x`: push every building with `left ≤ x`; pop from the top while the top's `right ≤ x` (finished buildings deeper in the heap can wait until they reach the top). The top's height, or 0, is the outline at `x`: record it if it changed.
