Product codes go up to 10⁹, so an array indexed by code is far too big. Which structure maps a code to a number of units, and returns 0 for codes it has never seen?
---
A hash map from item to units on the shelf. `"stock"` adds, `"count"` reads (0 when missing).
---
For `"sell"`, the number sold is `min(qty, units)`: report it and subtract it from the shelf.
