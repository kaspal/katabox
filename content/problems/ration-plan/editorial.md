## Approach: sort, prefix sums, binary search

**Cheapest first.** If a team can afford some `k` packs, it can afford the `k` cheapest ones too: swapping any chosen pack for a cheaper unchosen one never raises the cost. So the most packs a budget buys is the largest `k` such that the `k` cheapest prices add up to at most the budget.

**Prepare once.** The stock is the same for every team, so sort the prices once and build the running totals:

```
total[0] = p[0],   total[i] = total[i − 1] + p[i]     (p sorted, cheapest first)
```

`total[k − 1]` is the cost of the `k` cheapest packs. Prices are positive, so the totals strictly grow, and the answer for a budget is **the number of totals that are at most the budget**. Because the list is sorted, a binary search finds that number in O(log n).

**Worked example** (Example 1). Sorted prices `3 3 5 7 12`, totals `3 6 11 18 30`. Budget 10: two totals (3 and 6) are within it, so 2. Budget 18: four totals, since 18 itself counts (`≤`, not `<`). Budget 2: none, so 0. Budget 30: all five.

**Why it is correct:** the exchange argument shows the `k` cheapest packs are the cheapest way to get `k` packs; the totals are increasing, so the `k` values that fit form a prefix of the list, and counting the totals within the budget gives its length.

**Complexity:** O(n log n) to sort and O(n) for the totals, once; O(log n) per budget. O((n + Q) log n) in all, with O(n) memory.

**Why the obvious way is too slow:** walking the sorted prices for each budget until the money runs out is O(n) per team. With 5·10⁴ packs and 5·10⁴ teams whose budgets reach most of the total, that is up to 2.5·10⁹ steps.

**Practical notes:** read all tokens at once and print all answers in one write. The totals reach 5·10⁴ · 10⁹ = 5·10¹³, beyond 32 bits: use `u64` or `i64` in Rust. In TypeScript a `Number` holds them exactly (they stay below 2⁵³), and so does a PHP integer on a 64-bit build. In TypeScript, sort numbers with `(a, b) => a - b`: the default `sort()` compares them as text. Search for the first total **greater** than the budget (an upper bound); its position is the answer.
