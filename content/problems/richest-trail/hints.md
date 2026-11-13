Every trail has a highest cave where it bends. Fix that cave: the best trail bending there goes down the best way on each side.
---
For each node, compute `down(v)`: the best total of a trail that starts at v and only goes downwards. A branch with a negative total is better skipped.
---
`down(v) = val + max(0, down(left), down(right))`. A trail bending at v is worth `val + max(0, down(left)) + max(0, down(right))`; keep the largest over all nodes. Start the answer at −∞, not 0, since all values may be negative.
