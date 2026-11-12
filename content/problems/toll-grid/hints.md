Without the coupon this is a cheapest-path problem with positive costs on the cells, which Dijkstra's algorithm solves. The coupon makes the cost of a cell depend on a decision made along the way.
---
Make the decision part of the position: the state is `(cell, coupon used?)`. That is two copies of the grid. Moving into a cell costs its toll and keeps the coupon state; if the coupon is still unused, you may instead move into the cell paying half its toll and switch to the "used" copy.
---
Run Dijkstra on these 2 · rows · cols states with a min-heap. Start with `(0, 0)` unused at `tolls[0][0]` and used at `tolls[0][0] // 2`. The answer is the cheaper of the two states of the bottom-right cell.
