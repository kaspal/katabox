## Approach: backtracking, one category at a time

Build the outfit category by category with a recursive function. `choose(c, spent)` returns how many ways the categories `c, c + 1, …` can be completed without exceeding the budget, given that `spent` has already been spent:

```
choose(c, spent):
    if c == number of categories: return 1          # a complete outfit
    total = 0
    for price in sorted prices of category c:
        if spent + price + cheapest(c + 1 ..) > budget: break
        total += choose(c + 1, spent + price)
    return total
answer = choose(0, 0)
```

`cheapest(c + 1 ..)` is the sum of the cheapest item of every later category: if even those do not fit, no completion does. Because each category is sorted, once one item is too expensive every later item is too, so the loop stops. This **pruning** is what makes backtracking faster than listing every outfit, although with at most 6⁶ = 46 656 outfits even the full list is small.

### Walkthrough

`prices = [[30, 50], [20, 40, 60], [10]]`, budget 100. With the 30 hat: jackets 20, 40 and 60 all fit (with the 10 shoes: 60, 80, 100), 3 outfits. With the 50 hat: 20 and 40 fit (80, 100), but 50 + 60 + 10 = 120 does not, so the loop stops: 2 outfits. Total 5.

**Complexity:** O(number of outfits that fit) recursive calls in the worst case, at most 6⁶, with recursion depth equal to the number of categories.
