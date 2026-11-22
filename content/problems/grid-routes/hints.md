Try a floor with no shelves at all, such as `floor = ["..", "..", ".."]`. Count the routes by hand and compare.
---
Without the memo the recursion would be correct, only slow. Look at what the memo uses as its key: do two different cells ever share one?
---
The memo is keyed by `r + c`, so every cell on the same diagonal shares one entry, and the first value stored is reused for cells with a different number of routes. Key it by the cell itself, `(r, c)` (in TypeScript, for example `r * cols + c`).
