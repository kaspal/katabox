Walking up from 1 and skipping the shelf's numbers answers one question, but with 10⁵
questions over 10⁵ folders it is far too slow. Forget the rotation for a moment: on a
sorted shelf, how many numbers below the `i`-th folder are free?
---
On the sorted shelf `s`, the `i`-th folder (from 0) has `i` taken numbers below it, so
`s[i] − (i + 1)` numbers below `s[i]` are free. That count never decreases as `i` grows,
so you can binary search it.
---
Find the rotation point with the base problem's search (or sort a copy). For each `k`,
binary search the first `i` with `s[i] − (i + 1) ≥ k`; the answer is `k + i` (`i = n` when
no folder has that many free numbers below it).
