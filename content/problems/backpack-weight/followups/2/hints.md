The base problem's table has one cell per capacity. With `capacity` up to 10⁹ that table
is far too big. Which quantity in this problem is small?
---
The total worth of all items is at most 100 · 100 = 10⁴. Index the table by worth instead
of by weight.
---
Let `lightest[v]` be the least total weight of a group of items worth exactly `v`
(`lightest[0] = 0`, the rest unreachable). Add the items one by one, going through the
worths from high to low as in the base problem. The answer is the largest `v` with
`lightest[v] ≤ capacity`.
