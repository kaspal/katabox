## The idea

For every position, find the next value to its right that is bigger, or `-1` if there is none. Scanning right from each position costs `O(n²)` on a falling list. A monotonic stack answers all positions in one pass.

Walk from left to right. The stack holds the positions that are still **waiting** for a bigger value. Their values decrease from the bottom of the stack to the top: if a bigger value had come after a waiting one, it would already have answered it. When a new value `a[i]` arrives, it is the answer for every waiting position on top whose value is smaller. Pop them one by one, writing `a[i]` as their answer, and stop at the first top that is not smaller: everything below it is bigger still. Then push `i`, which now waits too. Positions left on the stack at the end never met a bigger value and keep `-1`.

## Steps

1. Fill `result` with `-1` and start with an empty `stack`.
2. For each `i` from left to right:
3. While the stack is not empty and `a[stack[-1]] < a[i]`, pop the top position and set its `result` to `a[i]`.
4. Push `i`.
5. After the last value, return `result`; positions still on the stack keep `-1`.

## Cost

| Case | Cost |
|---|---|
| Best | `O(n)` (rising list: each value pops the one before it) |
| Average | `O(n)` |
| Worst | `O(n)`: each position is pushed once and popped at most once |
| Extra space | `O(n)`: the stack (a falling list keeps every position on it) |

As with a sliding window, the inner loop does not make it `O(n²)`: across the whole pass there are only `n` pushes, so there can be at most `n` pops.

## Pitfalls

- **Storing values instead of positions.** The sample writes the answer into `result[stack.pop()]`, which needs the position. Keep positions on the stack and read their values through `a`.
- **`<` against `<=`.** With `<`, an equal value does not count as greater, so equal values wait together. Use `<=` only if the task wants the next value that is greater **or equal**.
- **Forgetting the leftovers.** Values that stay on the stack have no answer. Pre-filling `result` with `-1` covers them without a second loop.

## When to use it

Use it whenever each element needs its nearest bigger (or smaller) neighbour: how many days until a warmer one, which building first blocks the view to the right, the span of a stock price, or the largest rectangle under a bar chart. For "previous greater", walk the same way but read the top of the stack before pushing; for "next smaller", flip the comparison.

## Related topics

- [Sliding window](learn:searching/sliding-window): another pass where each index enters and leaves once.
- [Amortised cost: the dynamic array](learn:foundations/dynamic-array): the same "cheap on average over the whole run" argument.
- [Recursion and the call stack](learn:foundations/recursion-call-stack): another stack, one frame per call.
- Monotonic queue (coming soon): the same idea with removals at both ends.
