Trying every pair works but is O(n²). The list is sorted: how can you use that?
---
Start with the cheapest and the most expensive item. If their total is over the budget, can the most expensive item be part of any valid pair with a cheaper partner than the cheapest?
---
Keep two indices, `lo` at the start and `hi` at the end. If `prices[lo] + prices[hi]` fits, record it and move `lo` right (a bigger partner might still fit); otherwise move `hi` left. Stop when they meet.
