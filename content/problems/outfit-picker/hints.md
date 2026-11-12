The number of categories is not fixed, so nested loops (one per category) do not work. Think of choosing the outfit one category at a time.
---
Write a recursive function `choose(c, spent)` that tries every item of category `c`, adds its price, and calls `choose(c + 1, spent + price)`. When `c` passes the last category, one complete outfit has been built: count it if `spent ≤ budget`.
---
Prune: prices are positive, so once `spent` is over the budget no completion can fit. Sorting each category's prices lets the loop stop at the first item that is too expensive.
