## The idea

**Dynamic programming** solves a problem by answering smaller versions of it once, saving each answer and building bigger answers from the saved ones. It pays off when two things hold. **Overlapping subproblems**: the same smaller question comes up again and again. **Optimal substructure**: the answer for a big case is put together from the answers for smaller cases.

Fibonacci numbers show both. `fib(0) = 0`, `fib(1) = 1`, and every later number is the sum of the two before it. Plain recursion follows that rule literally: `fib(8)` calls `fib(7)` and `fib(6)`, but `fib(7)` calls `fib(6)` again, and the repeats multiply all the way down. The number of calls grows like the Fibonacci numbers themselves, about 1.6 times more for each step of `n`: 67 calls for `fib(8)`, over two million for `fib(30)`.

There are two ways to stop the waste. **Top-down (memoization)** keeps the recursion and adds a memo, a dictionary from `n` to its answer. A call first looks in the memo; if the answer is there, that is a **hit** and the recursion stops on the spot. Otherwise it works the answer out and **stores** it. **Bottom-up (tabulation)** drops the recursion: it fills a table from the smallest case upwards, `table[i] = table[i - 1] + table[i - 2]`, so every cell it reads is already filled.

The sample does both. It outputs `fib(n)` from the memo, then the whole table `table[0..n]`.

### The default input

`n = 8`. The memo version dives down the first branch: `fib(8)` calls `fib(7)`, which calls `fib(6)`, and so on until `fib(2)` calls `fib(1)` and `fib(0)`, which store 1 and 0. On the way back up, each `fib(k)` from 3 to 8 makes its second call, `fib(k - 2)`, and finds it in the memo. That is 9 stores and 6 hits: 15 calls instead of 67. The table version fills cells 2 to 8 from left to right, 7 additions in all.

The output is `[21, 0, 1, 1, 2, 3, 5, 8, 13, 21]`: first `fib(8) = 21`, then the table.

## Steps

1. Top-down: if `n` is in the memo, return the saved answer (a hit).
2. Otherwise the answer is `n` when `n < 2`, else `fib(n - 1) + fib(n - 2)`. Store it in the memo and return it.
3. Bottom-up: make a table of `n + 1` zeros and set `table[1] = 1` when `n > 0`.
4. For `i` from 2 to `n`, set `table[i] = table[i - 1] + table[i - 2]`.

## Cost

| Case | Cost |
|---|---|
| Plain recursion | `O(φⁿ)` calls, with `φ ≈ 1.618` |
| Memo or table | `O(n)`: each value is worked out once |
| Extra space | `O(n)`: the memo and the call stack, or the table |

## Pitfalls

- **Checking a memo that is never written.** Without the store, it is plain recursion again.
- **Deep recursion.** Top-down goes `n` calls deep, and Python stops at about 1000 by default; the table has no such limit.
- **Filling in the wrong order.** Bottom-up only works if every cell a formula reads was filled earlier. Here that means left to right.

## When to use it

Reach for dynamic programming when a recursive answer keeps asking the same questions. Top-down is handy when the recursion is already written or only some subproblems are needed; bottom-up avoids the call stack and makes the fill order explicit. When each cell reads only the last few, keep just those: two variables are enough for Fibonacci.

## Related topics

- [Recursion and the call stack](learn:foundations/recursion-call-stack): the calls the memo cuts short.
- [Big-O growth curves](learn:foundations/big-o-growth): exponential against linear growth.
- [Climbing stairs](learn:dynamic-programming/climbing-stairs): the same rule in disguise.
- Longest common subsequence (coming soon): a table with two dimensions.
