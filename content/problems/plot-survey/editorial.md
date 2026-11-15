## Approach: shoelace formula and Pick's theorem

The obvious method tests every whole-number point of the bounding box against the fence. A plot can span the whole field, so its box holds up to 2·10⁶ × 2·10⁶ = 4·10¹² points, each checked against up to 1000 edges: hopeless. Its cost depends on the area, not on `n`. Two classic formulas need only the corners, so the work is O(n).

**The area: shoelace formula.** Going round the corners, `x₁·y₂ − x₂·y₁` is twice the signed area of the triangle formed by the origin and the edge from `(x₁, y₁)` to `(x₂, y₂)`. Summed over all edges, the parts outside the plot cancel and what is left is twice the plot's area, positive when the corners go anticlockwise and negative when they go clockwise:

```
twice = |Σ (xᵢ·yᵢ₊₁ − xᵢ₊₁·yᵢ)|      (the corner after the last is the first)
```

`twice` is a whole number, so the area is `twice / 2`, ending in `.5` exactly when `twice` is odd. Print it from the integer (`twice // 2`, then `.0` or `.5`) and floating point never gets involved.

**The fence points.** On an edge with steps `dx = x₂ − x₁` and `dy = y₂ − y₁`, the whole-number points are spaced `(dx/g, dy/g)` apart, where `g = gcd(|dx|, |dy|)`: there are `g + 1` of them counting both ends. Counting one end per edge (the other is the next edge's start) gives `B = Σ gcd(|dx|, |dy|)`. A corner on a straight stretch is fine: its two edges simply split those points.

**The inside points: Pick's theorem.** For a simple polygon whose corners are whole-number points, `A = I + B/2 − 1`. Solving for `I`:

```
I = (twice − B + 2) / 2
```

Example 2: `twice = |0 + 15 + 0| = 15`, so the area is 7.5. The edges give `B = gcd(5, 0) + gcd(5, 3) + gcd(0, 3) = 5 + 1 + 3 = 9`, and `I = (15 − 9 + 2) / 2 = 4`.

**Why it is correct:** the shoelace sum is exact for any simple polygon, in either direction (the absolute value removes the sign). Pick's theorem holds for every simple polygon with whole-number corners, convex or not; the constraints guarantee exactly that, so `twice − B + 2` is always even.

**Complexity:** O(n log C) time for the `n` gcds (with `C` = 2·10⁶), O(n) memory.

**Number sizes:** each term `x₁·y₂ − x₂·y₁` is at most 2·10¹² in size, and `twice` at most 8·10¹². Use `i64` in Rust. In TypeScript every value stays far below 2⁵³, so `Number` is exact; PHP integers are 64-bit. Rust, TypeScript and PHP have no built-in gcd for plain integers: write Euclid's loop (`gcd(a, 0) = a`, and an edge is never `(0, 0)` because the corners are different).

**Input:** read all tokens at once (`sys.stdin.buffer.read().split()` and the like).
