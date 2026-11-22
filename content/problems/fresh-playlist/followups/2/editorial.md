## Idea

**Greedy is optimal.** Cut each block as long as possible. Any valid split's first block
ends no later than the greedy one, and a fresh stretch stays fresh when it is shortened,
so by induction the greedy cuts are always at least as far along as any other split's: it
never uses more blocks.

**The next cut.** A block starting at `i` can run to play `r` exactly when the longest fresh
stretch ending at `r` starts at or before `i`, that is `start[r] ≤ i` (with `start` from
the base problem's window, never decreasing). So `next[i]`, the first play after the
longest block from `i`, is the first `r` with `start[r] > i` (or `n`); one pointer finds
all of them.

**Many requests.** Build `jump[0] = next` and `jump[k][i] = jump[k−1][jump[k−1][i]]`, with
`n` jumping to itself. For a request `[a, b]`, start at `a` with one block; for `k` from
high to low, if `jump[k][pos] ≤ b`, move there and add `2^k` blocks. The final count is the
answer.

## Complexity

O(n log n) time and memory to build the jumps, O(log n) per request. Walking the greedy
blocks per request is up to O(n) each, 10¹⁰ steps in all.
