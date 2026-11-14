## The idea

A recursive function solves a problem by calling itself on smaller parts of it. Here `total(a, lo, hi)` sums the range `lo..hi`: one value is its own sum (the **base case**); a longer range splits at `mid` and adds the sums of its two halves. Every call gets a **frame** on the call stack holding its own `lo`, `hi`, `mid`, `left` and `right`. A frame waits on the stack until the calls it made have returned, then pops. The tree on the right records every call that ever ran.

## Steps

1. Call `total(a, lo, hi)`: a new frame goes on top of the stack.
2. If `lo == hi`, return `a[lo]`: the frame pops and its caller resumes.
3. Otherwise set `mid = (lo + hi) // 2`.
4. Call `total(a, lo, mid)` and wait for its result in `left`.
5. Call `total(a, mid + 1, hi)` and wait for its result in `right`.
6. Return `left + right`; the frame pops.

## Cost

| Measure | Cost |
|---|---|
| Time | `O(n)`: `2n − 1` calls, each doing constant work |
| Calls | `n` base cases plus `n − 1` splits |
| Stack depth | `O(log n)`: halving reaches one value after about `log₂ n` splits |
| Extra space | `O(log n)` frames at any moment |

## Pitfalls

- **A missing or unreachable base case.** If the halves overlapped as `lo..mid` and `mid..hi`, the range `lo..lo + 1` would call itself with the same range forever. Every call must shrink the range until it hits `lo == hi`.
- **Stack overflow.** Each waiting call costs a frame. Peeling one value at a time (`a[lo] + total(a, lo + 1, hi)`) gives depth `n`, and Python stops at about 1 000 frames by default. Halving keeps the depth at `log n`.
- **Thinking the halves run at once.** The left call finishes completely, all its own calls included, before the right call starts. Watch the stack: only the top frame is running.

## When to use it

Recursion fits problems that split into smaller copies of themselves: ranges cut in halves, trees, nested data, and searches that try a choice and back up. When the depth could reach the thousands, rewrite it with a loop and an explicit stack.

## Related topics

- [Towers of Hanoi](learn:foundations/towers-of-hanoi): recursion where each call makes two smaller ones, with `2ⁿ − 1` moves.
- [Quick Sort](learn:sorting/quick-sort): split a range and recurse on both sides.
- [Big-O growth curves](learn:foundations/big-o-growth): where `log n` comes from.
- [Merge Sort](learn:sorting/merge-sort): the same halving split, merging instead of adding.
- Binary search (coming soon): halving, but only one side is visited.
