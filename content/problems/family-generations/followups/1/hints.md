If you know each person's depth (their generation), the distance is depth(a) + depth(b) − 2 · depth(c), where c is the closest ancestor they share. The whole problem is finding c quickly.
---
Walking up one parent at a time can take 10⁵ steps per pair when the tree is a long chain. What if every person also knew their ancestor 2, 4, 8, … generations up?
---
Build a table `up[k][v]`: the ancestor 2ᵏ generations above v (the founder points to itself). Lift the deeper person to the other's depth using the binary digits of the difference; if both are now the same person, that is c. Otherwise, from the largest k down, jump both whenever `up[k]` differs for them; then their parent is c.
