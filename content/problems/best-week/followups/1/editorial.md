## Idea

Let `prefix[j]` be the total of the first `j` days. A run of days `i..j−1` totals
`prefix[j] − prefix[i]`, and it has 1 to `k` days exactly when `j − k ≤ i ≤ j − 1`. So for
each end `j`, the best run subtracts the smallest prefix in that sliding window of starts.
A monotonic deque gives that minimum: before adding start `j − 1`, pop from the back every
index whose prefix is not smaller, since the new start is both later and at least as low;
then drop the front if it has left the window. The front is the minimum, and the best of
`prefix[j] − prefix[front]` over all `j` is the answer. Starting the best at the first run
(not at 0) keeps all-loss inputs right.

## Complexity

O(n) time, since each index enters and leaves the deque once, and O(n) extra memory for the
prefix sums and the deque. Totals stay within ±2·10⁹, which fits in a signed 32-bit
integer, but 64-bit arithmetic is the safe habit.
