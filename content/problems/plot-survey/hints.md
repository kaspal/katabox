Testing every whole-number point of the plot is hopeless: a plot can hold about 10¹² of them. Look for formulas that use only the `n` corners: one for the area, and one that links the area to the points inside.
---
The **shoelace formula** gives twice the area as a whole number: `|Σ (xᵢ·yᵢ₊₁ − xᵢ₊₁·yᵢ)|`, going round the corners. **Pick's theorem** links the area `A` of a polygon with whole-number corners to its inside points `I` and the whole-number points on its fence `B`: `A = I + B/2 − 1`.
---
For each edge from `(x₁, y₁)` to `(x₂, y₂)`, add `x₁·y₂ − x₂·y₁` to `twice` and `gcd(|x₂ − x₁|, |y₂ − y₁|)` to `B` (the fence points of that edge, counting one of its ends). Then `twice = |twice|`, the area is `twice / 2` (`.5` when `twice` is odd), and `I = (twice − B + 2) / 2`.
