## Approach: sort, then two pointers from both ends

Checking every pair costs `n(n − 1)/2` steps per case. The input may be a single case of 10⁵ teams: that is about 5·10⁹ pairs, far too many for the time limit in any language. The many-small-cases tests are no problem for it; the one big case is.

**Sort first.** Which pairs fit does not depend on the order of the balances, so sort them: `a[0] ≤ a[1] ≤ … ≤ a[n−1]`. Now look at the smallest and the largest that are still in play, `a[i]` and `a[j]`:

- if `a[i] + a[j] ≤ K`, then `a[i]` fits with **every** `a[i+1..j]`, since each of them is at most `a[j]`: that is `j − i` pairs. Count them and drop `a[i]` (`i += 1`);
- otherwise `a[j]` does not fit even with the smallest one left, so it fits with none of them: drop `a[j]` (`j -= 1`).

Stop when `i` meets `j`. For the first case of Example 1, sorted `2 3 5 8` with `K = 10`:

```
i=0 j=3   2 + 8 = 10 ≤ 10   count += 3   (2 with 3, 5, 8)
i=1 j=3   3 + 8 = 11 > 10   drop 8
i=1 j=2   3 + 5 = 8  ≤ 10   count += 1   (3 with 5)
i=2 j=2   stop               count = 4
```

**Why it is correct:** every step drops one balance after settling all its pairs with the balances still in play, so each pair is decided exactly once, at the moment the first of its two members is dropped. The pairs with balances dropped earlier were settled at that earlier step. The two rules above show that the pairs settled at each step are counted correctly.

**Complexity:** O(n log n) for the sort and O(n) for the pointers, per case. Since the sum of `n` over all cases is at most 10⁵, the total is O(Σn · log Σn), however the teams are split into cases.

**Several test cases:** read all tokens at once and walk them with a position: `T`, then for each case `n`, `K` and the next `n` balances. Start each case afresh (the count back at 0, the case's own balances only), and do not allocate an array of 10⁵ for every case: with 10⁴ cases that alone would be 10⁹ cells. Collect the answers and print them once.

**Integer traps:** the count can reach about 5·10⁹, more than a 32-bit integer holds: use `i64` or `u64` in Rust. A sum of two balances reaches ±2·10⁹, which only just fits in 32 bits; `i64` everywhere is the safe choice. TypeScript `Number` is exact for all of these, but `Array.prototype.sort()` without a comparator sorts **as strings** (`-1` before `-10`, `10` before `9`): pass `(x, y) => x - y` or use a `Float64Array`, whose `sort()` is numeric. In PHP, convert the tokens with `intval` (or `(int)`) before calling `sort`.
