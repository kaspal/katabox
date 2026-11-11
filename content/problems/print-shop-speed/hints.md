For a given speed, can you compute the total hours quickly? How does that total change when the speed goes up?
---
A faster printer never needs more hours. So the speeds that work form a range: every speed from the answer upwards works, every slower one does not.
---
Binary search the speed between 1 and `max(pages)` (that speed always works). For the middle speed, add up ⌈p / speed⌉; if it fits in `hours`, search lower, otherwise higher.
