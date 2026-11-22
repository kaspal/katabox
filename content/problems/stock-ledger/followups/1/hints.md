A Fenwick tree that updates one shelf at a time would need up to 10⁵ updates per
delivery. Can a delivery to a whole row be recorded with a constant number of point
updates?
---
Record a delivery as `+count` at shelf `a` and `−count` just after shelf `b` in a
difference array: the prefix sum of that array at shelf `i` is how much shelf `i` got.
Now you need sums of those prefix sums.
---
The total of shelves `0..i−1` equals `i · D(i) − W(i)`, where `D(i)` is the sum of the
differences at positions below `i` and `W(i)` is the sum of each difference times its
position. Keep two Fenwick trees, one for each, and answer `total(a, b)` as
`prefix(b + 1) − prefix(a)`.
