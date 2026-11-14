## The idea

A **deque** (double-ended queue) lets you push and pop at both the front and the back, each in `O(1)`. Its best-known trick is the **monotonic queue**, which finds the maximum of every window of `k` values in a single pass.

The deque holds **positions**, not values, and keeps their values decreasing from front to back. When a new value arrives:

- Every value at the back that is **smaller than or equal to** it can never be a window's maximum again: the new value is at least as big and stays in the window longer. Pop them off the back, then push the new position.
- If the position at the front has slid out of the window, pop it off the front.
- The front is now the largest value in the window.

### Why it is linear

A single arrival can pop several values, so one step may look expensive. But each position is pushed exactly once and popped at most once, so all the pops together cost at most `n`. The whole pass is `O(n)`, compared with `O(n · k)` for scanning each window.

## Steps

1. Start with an empty deque `dq` and an empty output row.
2. For each position `i`: while `dq` is not empty and `a[dq.back] <= a[i]`, pop the back.
3. Push `i` onto the back.
4. If the front position is `<= i − k`, it is outside the window: pop the front.
5. Once `i >= k − 1`, the window `i − k + 1..i` is complete: output `a[dq.front]`.

## Cost

| Measure | Cost |
|---|---|
| Push or pop at either end | `O(1)` |
| Whole pass over `n` values | `O(n)`: each position enters and leaves once |
| Extra space | `O(k)`: the deque holds at most `k + 1` positions, briefly |

## Pitfalls

- **Storing values instead of positions.** With values alone you cannot tell when the front has left the window. Keep positions and read the values through them.
- **Popping only strictly smaller values.** Using `<` keeps equal values around. The answer stays right, but the deque grows for nothing; `<=` keeps it lean.
- **A list as the deque.** Removing the front of a Python list or a JavaScript array is `O(n)`. Use `collections.deque`, or a head index into an array.

## When to use it

Use a monotonic queue for maxima or minima of a window that slides one step at a time: the hottest hour in every 3-hour span, the cheapest price in the last week. For the minimum, flip the comparison so the values increase from front to back.

## Related topics

- [Sliding window](learn:searching/sliding-window): the window that this deque keeps the maximum of.
- [Monotonic stack](learn:searching/monotonic-stack): the same pruning with only one open end.
- [Queue and circular buffer](learn:linear/queue-ring-buffer): a plain queue in a fixed array.
- [Doubly linked list](learn:linear/doubly-linked-list): one way to build a deque.
- Binary heap (coming soon): window maxima in `O(n log n)`.
