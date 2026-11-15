Fix the row where the banner's bottom edge sits. For each column, how many free squares stand in an unbroken run that ends on that row? With those heights the wall above the row looks like a skyline of bars.
---
The heights update in O(C) per row: a free square adds 1 to the height above it, a blocked one resets it to 0. The best banner whose bottom edge is on this row is the largest rectangle under the skyline. For each bar, the widest rectangle of its height stretches until the first lower bar on each side.
---
Sweep the bars left to right with a stack of columns whose heights increase. Before pushing column `x`, pop every column at least as tall as `height[x]`: a popped bar of height `h` reaches from just after the column now on top of the stack (or from column 0 if the stack is empty) up to `x − 1`, so its area is `h · (x − left)`. Add a bar of height 0 at the end to empty the stack. Keep the best area over all rows.
