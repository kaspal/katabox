Only the trees at the "corners" of the fence matter. If you had them in order around the fence, the shoelace formula would give twice the area directly.
---
Sort the trees by x (then y). Walking from left to right, the lower half of the fence only ever turns left; a tree that would make it turn right (or go straight) cannot be a corner and is dropped.
---
Andrew's monotone chain: build the lower hull left to right and the upper hull right to left, popping the last corner while `cross(second-last, last, new) ≤ 0`. Then sum `x_i·y_{i+1} − x_{i+1}·y_i` around the hull.
