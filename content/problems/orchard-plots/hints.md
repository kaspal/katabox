Adding up every tree of every plot repeats a lot of work when there are many large plots. In one dimension, how do prefix sums answer "total of a stretch" in O(1)?
---
Let `pre[r][c]` be the total of the rectangle from the top-left corner down to row `r − 1` and column `c − 1`. Any plot is a combination of four such corner rectangles.
---
plot = pre[bottom+1][right+1] − pre[top][right+1] − pre[bottom+1][left] + pre[top][left]. Build `pre` row by row: `pre[r+1][c+1] = pre[r][c+1] + (running total of row r up to column c)`.
