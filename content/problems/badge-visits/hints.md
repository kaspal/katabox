Recounting the distinct ids of the first `p` scans for every question repeats almost the same work again and again: up to 30 000 questions over up to 30 000 scans is far too much.
---
The answer depends only on `p`, and it never shrinks as `p` grows: scan `p` adds one new member exactly when its id did not appear in scans `1 .. p-1`.
---
Walk the scans once with a set of the ids seen so far, and store `distinct[p]`, the size of the set after scan `p`. Every question is then a single lookup: print `distinct[p]`.
