With at most 15 dishes there are at most 2¹⁵ = 32 768 sets. How can you walk through all of them without recursion?
---
Use a number `mask` from 1 to 2ⁿ − 1 as a set: dish `i` is in the set when bit `i` of `mask` is 1, that is, when `mask & (1 << i)` is not 0.
---
For each mask, add up the prices of its dishes; if the total is the budget, collect the dish numbers (in increasing order of `i`). Masks do not come out in lexicographic order, so sort the collected lists at the end.
