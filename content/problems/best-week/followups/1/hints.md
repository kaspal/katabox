With losses, a longer run is not always better, so a fixed window of `k` days no longer
works. Trying every length from 1 to `k` at every end is O(n · k). What does the sum of a
run look like in terms of prefix sums?
---
The run ending at day `j − 1` and starting at day `i` totals `prefix[j] − prefix[i]`. For
a fixed end, you want the smallest `prefix[i]` with `j − k ≤ i ≤ j − 1`: a sliding-window
minimum.
---
Keep a deque of prefix indexes whose prefix values increase from front to back. For each
end `j`, push `j − 1` after popping larger-or-equal values from the back, drop the front
if it is older than `j − k`, and compare `prefix[j] − prefix[front]` with the best.
