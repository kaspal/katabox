Trying every pair of pivots is about 5·10⁹ pairs. With prefix sums, each of the three
stretch totals takes O(1); can you also avoid looping over the first pivot?
---
Write the three totals with prefix sums `P` (`P[k]` = sum of the first `k` crates):
left = `P[i]`, middle = `P[j] − P[i] − weights[i]`, right = `total − P[j + 1]`. Fix `j`
and ask what an earlier `i` must look like.
---
For a fixed `j`, call `R` the right total. A good `i` needs `P[i] = R` and
`weights[i] = P[j] − 2R`. Walk `j` from left to right, adding each `i = j − 1` to a map
keyed by the pair `(P[i], weights[i])`, and add the count stored under
`(R, P[j] − 2R)`.
