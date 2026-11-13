## Approach: four orientations

Let `o(A, B, C)` be the sign of the cross product `(B − A) × (C − A)`: +1 if C is left of A→B, −1 if right, 0 if on the line.

Compute `d1 = o(P1, P2, P3)`, `d2 = o(P1, P2, P4)`, `d3 = o(P3, P4, P1)`, `d4 = o(P3, P4, P2)`.

1. **Proper crossing:** if `d1` and `d2` are non-zero with opposite signs, and so are `d3` and `d4`, the paths cross at an interior point.
2. **Touching or overlapping:** otherwise, the paths meet only if some endpoint lies **on** the other path: its orientation is 0 and it is inside the other path's bounding box. For instance `d1 = 0` and P3 within the box of P1, P2.
3. Otherwise they do not meet.

Collinear paths are covered by step 2: they overlap exactly when an endpoint of one lies within the other.

**Complexity:** O(1) per pair, all in integers.
