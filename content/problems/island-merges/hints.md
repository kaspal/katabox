Recounting the islands with a flood fill after every square costs the whole of the land each time. A new square only changes things around itself: by how much can the count change?
---
A new square is a new island (+1). Each **different** island among its four side neighbours then merges with it (−1 each). You need a fast answer to "are these two squares already on the same island?": union-find.
---
Number the squares `r·(C + 2) + c`, with a border of sea all round so neighbours never fall off the map. Keep `parent[]`, with −1 for sea. For a new square: `parent[cell] = cell`, `islands += 1`; for each land neighbour, find both roots (a loop with path halving) and, if they differ, link one to the other and `islands −= 1`. A square that is already land prints the count unchanged.
