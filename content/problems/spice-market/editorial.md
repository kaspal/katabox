## Approach: best value per gram first (fractional knapsack)

Since sacks can be split, the bag is just `capacity` grams to fill, and each gram should hold the most valuable spice still available. So take spices in decreasing order of value per gram:

```
sort stalls by prices[i] / weights[i], highest first   # compare p1·w2 > p2·w1
total = 0; room = capacity
for each stall (w, p):
    if w <= room: total += p; room -= w
    else: total += p * room / w; break
```

**Why it is optimal.** Suppose a better haul carries some grams of a spice `b` while a spice `a` with a higher value per gram is not fully taken. Swapping one of those grams of `b` for a gram of `a` keeps the weight and does not lower the value. Repeating the swap turns any haul into the greedy one without losing value.

Only the last sack is split, so the answer is a whole number plus one fraction. Comparing ratios by cross-multiplication (`p1 · w2` against `p2 · w1`, at most 10⁸) avoids rounding in the sort.

### Walkthrough

Values per gram 6, 5, 4. Take 10 g (60), then 20 g (100); 20 g of room remain, so take 20/30 of the third sack: 120 · 20 / 30 = 80. Total 240.

**Complexity:** O(n log n) for the sort and O(n) for the scan.
