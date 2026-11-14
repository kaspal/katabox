## The idea

Big-O describes how the work of an algorithm **grows** as its input grows, ignoring constant factors. The animation runs six small loops on the same `n` and counts their `steps`: one step, a doubling loop, a loop over `n`, a doubling loop inside a loop over `n`, a pair of nested loops, and a loop over every subset of `n` items. The counts start close together and then drift apart very fast; the chart shows each count as a curve over `n`.

## Steps

1. `constant`: one step, whatever `n` is.
2. `logarithmic`: `i` starts at 1 and doubles while `i <= n`, so it reaches `n` after about `log₂ n` steps.
3. `linear`: one step per value, `n` steps.
4. `linearithmic`: the doubling loop runs once for each of the `n` values: about `n log n` steps.
5. `quadratic`: one step for every pair `(i, j)`: `n²` steps.
6. `exponential`: one step for each of the `2ⁿ` subsets.
7. Move the `n` slider and watch the ranking of the curves settle.

## Cost

| Growth | Steps at n = 10 | Steps at n = 16 |
|---|---|---|
| `O(1)` | 1 | 1 |
| `O(log n)` | 4 | 5 |
| `O(n)` | 10 | 16 |
| `O(n log n)` | 40 | 80 |
| `O(n²)` | 100 | 256 |
| `O(2ⁿ)` | 1 024 | 65 536 |

## Pitfalls

- **Trusting small `n`.** At `n = 3` the quadratic loop counts 9 steps and the exponential one only 8. Big-O is about what happens as `n` grows: from `n = 5` on, `2ⁿ` is always ahead.
- **Multiplying every nested loop by `n`.** The inner loop of `linearithmic` doubles `j`, so it adds `log n` steps per round, not `n`. Look at how each loop variable changes.
- **Reading constants into Big-O.** `2n` and `n + 100` are both `O(n)`, and the base of the logarithm does not matter. Constants still count when you compare two algorithms of the same class.

## When to use it

Use Big-O before you write code: estimate `n` from the problem's limits and pick an algorithm whose growth fits. As a rough guide, a computer does around 10⁸ simple steps per second, so `n²` is fine for `n` in the thousands, `n log n` for millions, and `2ⁿ` only up to about 25.

## Related topics

- [Recursion and the call stack](learn:foundations/recursion-call-stack): where `log n` depth comes from.
- [Amortised cost: the dynamic array](learn:foundations/dynamic-array): cost averaged over many operations.
- [Quick Sort](learn:sorting/quick-sort): `O(n log n)` on average, `O(n²)` at worst.
- [Bubble Sort](learn:sorting/bubble-sort): a plain `O(n²)` algorithm.
