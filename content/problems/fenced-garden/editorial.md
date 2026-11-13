## Approach: boundary test, then ray casting

**On the fence?** For each side A→B, the point P is on it when `cross(A, B, P) = 0` (P is on the line) and P is inside the bounding box of A and B. Checking every side first settles the `"boundary"` case exactly.

**Inside or outside?** Cast a horizontal ray from P towards +x and count the sides it crosses. Inside points cross an odd number of times. To count each corner once, treat a side as crossing the ray only when one end is strictly above P and the other is not: `(A.y > P.y) != (B.y > P.y)`. That half-open rule counts a corner shared by two sides exactly once, and ignores horizontal sides.

The crossing's x-coordinate is `A.x + (P.y − A.y)(B.x − A.x)/(B.y − A.y)`. To stay in integers, multiply out: the crossing is to the right of P when `t = (P.y − A.y)(B.x − A.x) − (P.x − A.x)(B.y − A.y)` is positive and `B.y > A.y`, or negative and `B.y < A.y`.

### Walkthrough

Point (3, 3) in the notched fence of Example 1: the ray to the right crosses the side from (6, 4) to (3, 2)? Its ends are at y = 4 and y = 2, so it straddles y = 3, but the crossing is at x = 4.5 > 3: one crossing. The side (6, 0)→(6, 4) is also crossed. Two crossings: outside.

**Complexity:** O(n) per point, O(n·m) in all; with n ≤ 300 and m ≤ 1000, that is 3·10⁵ steps.
