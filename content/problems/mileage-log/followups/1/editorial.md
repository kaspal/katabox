## Idea

Answer the questions **offline**, in an order that suits us. Sort the questions by `x`
from largest to smallest, and the days by kilometres from largest to smallest. Keep a
Fenwick tree over the day positions, all zeros at first.

For each question in that order, first add 1 at the position of every day not yet added
whose kilometres are at least `x` (a pointer moves along the sorted days, so each day is
added once). Now the tree holds a 1 exactly at the long days for this threshold, and the
answer is the sum over positions `a` to `b`: two prefix sums. Write it at the question's
original index.

Because the kilometres are at most 1000, another route works too: split the days into
blocks of about √n and keep, per block, a count for each kilometre value. That is also far
faster than counting day by day.

## Complexity

O((n + q) log n) time with the sorting and the Fenwick tree, O(n + q) extra memory. The
day-by-day count is O(n · q), up to 10¹⁰ steps.
