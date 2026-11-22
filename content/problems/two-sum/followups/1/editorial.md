## Idea

The base problem stopped at the first pair; here every pair counts. Scan left to right
with a hash map `seen` from value to how many times it appeared so far. When you reach a
number `x`, each earlier `target − x` forms one pair with it, so add `seen[target − x]`
to the answer, then record `x`. Counting before recording keeps `i < j` and never pairs a
position with itself.

## Complexity

O(n) time and O(n) extra memory. The answer can reach about 5·10⁹, so use a 64-bit
integer (a JavaScript number is exact up to 2⁵³).
