Binary searching each order separately costs O(n log m) per order, about 10⁵ · 17 · 10⁵
steps in total. The rolls never change between orders: what could you compute once and
reuse for all of them?
---
Let `count[L]` be the number of pieces of length `L`. It only goes down as `L` grows, and
the rolls are at most 10⁵ long, so you can afford to know it for every `L`. Then each
order is a binary search over `count`.
---
A roll of length `r` gives one piece of length `L` for every multiple `k·L ≤ r`, so
`count[L]` is the sum over `k = 1, 2, …` of how many rolls are at least `k·L` long. A
prefix count of roll lengths answers each term in O(1), and summing over `L` gives
m/1 + m/2 + … ≈ m ln m terms.
