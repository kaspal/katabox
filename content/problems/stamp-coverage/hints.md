Inking every cell of every stamp can mean 10⁵ stamps × 2.5·10⁵ cells. In one dimension, how can you add 1 to a whole stretch with two writes?
---
A difference array: +1 at the start of the stretch and −1 just after it; a running sum then rebuilds the counts. In two dimensions, a rectangle needs four marks.
---
Mark +1 at (top, left), −1 at (top, right+1), −1 at (bottom+1, left), +1 at (bottom+1, right+1). Then take running sums along every row, then down every column; read off the maximum and how often it occurs.
