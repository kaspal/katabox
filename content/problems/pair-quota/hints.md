Trying every pair is `n(n − 1)/2` checks: about 5·10⁹ for one case of 10⁵ teams. The count does not depend on the order of the balances, so you are free to reorder them.
---
Sort the balances. If the smallest plus the largest fit within `K`, the smallest fits with **every** other balance too, since all of them are at most the largest. If they do not fit, the largest fits with nobody left.
---
Two pointers on the sorted balances, `i = 0` and `j = n − 1`: while `i < j`, if `a[i] + a[j] ≤ K` add `j − i` to the count and move `i` right; otherwise move `j` left. Do this separately for each case, starting the count at 0, and keep the count in a 64-bit integer.
