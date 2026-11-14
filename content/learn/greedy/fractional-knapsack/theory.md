## The idea

A knapsack holds at most a given **capacity** of weight. Each item has a weight and a value, and this time any **part** of an item may be packed: half of an item weighs half as much and is worth half as much. Think of sacks of rice or spice sold by the gram. What is the largest total value?

Each item has a **value per unit of weight**, its ratio `value / weight`. The greedy rule is to pack the item with the best ratio first, all of it if it fits, then the next best, and so on. The first item that does not fit is cut to fill the room exactly, and the knapsack is full. Ties in the ratio keep the input order.

Why is this safe? Suppose a load holds some weight of a poorer item while a richer one is not fully packed. Swap one unit of weight: out goes a unit of the poorer item, in goes a unit of the richer one. The weight stays the same and the value does not drop. Repeating the swap turns any load into the greedy one without losing value.

To stay exact, the sample returns the total value as a mixed number: `[whole, numerator, denominator]`, the integer part, then the leftover fraction in lowest terms. When every item fits, the fraction is `0/1`. The input is typed as the capacity followed by "weight value" pairs.

### The default input

Capacity 10; items `(4, 12)`, `(6, 9)`, `(3, 12)`, `(5, 5)` as weight and value. The ratios are 3, 1.5, 4 and 1, so the order is items 2, 0, 1, 3. Item 2 fits: value 12, room 7. Item 0 fits: value 24, room 3. Item 1 weighs 6 but only 3 is left, so half of it goes in: `9 · 3 / 6 = 27 / 6 = 4 1/2`. The total is `24 + 4 1/2 = 28 1/2`.

The output is `[28, 1, 2]`. Item 3 never comes into play.

## Steps

1. Sort the items by ratio, highest first; ties keep the input order.
2. Start with `room = capacity` and a value of 0.
3. For each item: if its weight fits in `room`, add its value and subtract its weight.
4. Otherwise add `value · room / weight` and stop: the knapsack is full.
5. Return the total as `[whole, numerator, denominator]`.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(n log n)`: the sort; the packing walk is `O(n)` |
| Extra space | `O(n)` for the sorted order |

## Pitfalls

- **Using it for the 0/1 knapsack.** With capacity 10 and items `(6, 12)`, `(5, 9)`, `(5, 9)`, the best ratio is the first item, but once it is in nothing else fits whole: 12, while the two others give 18. Greedy is right here only because items split.
- **Floating-point totals.** A part such as `1/3` has no exact floating-point value, and sums of such parts can drift. Keep a numerator and a denominator, as the sample does, or compare ratios by cross-multiplying.
- **Forgetting to stop.** After the cut item the knapsack is full; later items must not be added.

## When to use it

Use it whenever goods can be divided: buying spices or fuel by weight, spending a budget across tasks paid by the hour. The linked problem, Spice Market, is this pattern.

## Related topics

- [0/1 knapsack](learn:dynamic-programming/knapsack-01): items that cannot be split, solved with a table.
- [Rod cutting (unbounded knapsack)](learn:dynamic-programming/rod-cutting): whole pieces that may repeat.
- [Activity selection](learn:greedy/activity-selection): what makes a problem greedy-friendly.
