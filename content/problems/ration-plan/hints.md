To buy as many packs as possible, which packs should a team pick first? And since the stock is the same for every team, what could you prepare once, before reading the budgets?
---
The best `k` packs to buy are always the `k` cheapest. Sort the prices and add them up from the cheapest: the running totals only grow, so the answer for a budget is how many running totals are at most that budget.
---
Sort the prices and build `total[k]` = the sum of the `k + 1` cheapest (prefix sums). For each budget, binary search for the number of totals `≤ budget` (Python `bisect_right`, Rust `partition_point`, a hand-written search in TypeScript and PHP). That is O(n log n + Q log n).
