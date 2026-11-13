Use orientations: for a path P1→P2, the sign of `cross(P1, P2, Q)` says whether Q is left of it, right of it, or on its line.
---
In the general case the paths cross exactly when P3 and P4 are on opposite sides of P1P2, **and** P1 and P2 are on opposite sides of P3P4.
---
The special cases come from zeros: if an endpoint is on the other path's line, the paths meet exactly when that endpoint lies within the other path's bounding box. Check the four endpoints this way, after the general test.
