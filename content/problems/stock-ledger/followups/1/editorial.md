## Idea

Think of the shelves through a difference array `d`: a delivery `[a, b, count]` adds
`count` to `d[a]` and subtracts it from `d[b + 1]`, and a shelf's value is the sum of `d`
up to it. The total of the first `i` shelves is then
`Σ_{j<i} (i − j) · d[j] = i · Σ d[j] − Σ j · d[j]`. Keep one Fenwick tree for `d[j]` and
another for `j · d[j]`; each delivery is two point updates in each tree, and each prefix
total is two prefix queries. Load the starting stock as a delivery of `stock[i]` to the
single shelf `i`, and answer `[a, b]` as `prefix(b + 1) − prefix(a)`. (A segment tree with
lazy additions works too.)

## Complexity

O((n + q) log n) time for `n` shelves and `q` operations, and O(n) extra memory. Totals
can reach about 10¹⁴, so use 64-bit integers; in JavaScript, numbers stay exact up to 2⁵³.
