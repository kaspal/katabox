## Idea

The `k`-th best score so far is the smallest of the current top `k`, and the top `k` only
changes by one swap per game: a new score either beats the weakest of them or does not
matter. Keep the top `k` in a min-heap. While it holds fewer than `k` scores, push every
score and report `-1` (until it fills). After that, if the new score is greater than the
heap's minimum, replace the minimum with it. Either way, the heap's minimum is the answer
for that game. Ties are harmless: equal scores each sit in the heap.

## Complexity

O(n log k) time and O(k) extra memory, plus the O(n) output. Since scores are at most 300,
counting how many games got each score also works, in O(301) per game.
