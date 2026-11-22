## Idea

The base problem compared two sides; now three stretches must match. With prefix sums
`P`, the totals for pivots `i < j` are left `P[i]`, middle `P[j] − P[i] − weights[i]`
and right `R = total − P[j + 1]`. Setting left = right gives `P[i] = R`, and then
middle = right gives `weights[i] = P[j] − 2R`. So for each `j` you need the number of
earlier positions `i` with a given pair `(P[i], weights[i])`. Scan `j` from 1 to
`n − 1`; before checking `j`, add `i = j − 1` to a hash map counting those pairs, then
add the count of `(R, P[j] − 2R)` to the answer.

## Complexity

O(n) time and O(n) extra memory. Prefix sums stay within ±10⁹, but the answer can reach
about 5·10⁹ (all zeros), so count with a 64-bit integer.
