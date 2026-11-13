Prefix sums answer totals instantly, but one `"set"` would force you to rebuild them. You need a structure that is cheap to update **and** to query.
---
A Fenwick tree (binary indexed tree) keeps partial sums so that both "add d at position i" and "total of positions 0 to i" take O(log n).
---
Keep the current counts in an array. For `"set"`, add `count − current[i]` at `i` in the Fenwick tree. For `"total"`, answer `prefix(b + 1) − prefix(a)`.
