## Approach: backtracking, thickest first, with pruning

This is **bin packing**, which has no known fast exact algorithm, but 15 books are few enough to search carefully.

1. Sort the books from thickest to thinnest.
2. Start `best` with a good answer: **first fit decreasing** (each book on the first shelf where it fits, otherwise a new shelf).
3. Search recursively: `place(i, loads)` puts book `i` on each used shelf where it fits, or on one new shelf, then places book `i + 1`. When every book is placed, `best = min(best, shelves used)`.

Pruning makes it fast:

- **Bound:** a branch that already uses `best − 1` shelves cannot improve by opening a new one, and a branch using `best` is dropped.
- **Symmetry:** shelves with equal loads are interchangeable, so only the first of them is tried for a book.
- **Lower bound:** no answer can be smaller than ⌈total / width⌉; once `best` reaches it, the search stops.

Placing the thickest books first makes overloaded branches fail near the top of the tree, where cutting saves the most.

### Walkthrough

`[4, 4, 4, 3, 3, 3, 2, 2, 2]`, width 10, total 27, so the lower bound is 3. First fit decreasing gives {4, 4}, {4, 3, 3}, {3, 2, 2}, {2}: 4 shelves. The search then finds {4, 4, 2}, {4, 3, 3}, {3, 2, 2}, which meets the lower bound, and stops.

**Complexity:** exponential in the worst case (at most 15! orders, far fewer with the pruning), with recursion depth n.
