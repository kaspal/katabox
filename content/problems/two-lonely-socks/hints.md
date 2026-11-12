XOR all the ids together, as for a single lonely sock. Pairs cancel out. What is left?
---
You get `a XOR b`, where `a` and `b` are the two lonely ids. It is not zero, because `a ≠ b`. Pick any bit that is 1 in it: `a` and `b` differ in that bit.
---
Split the ids by that bit and XOR each group separately. Every pair lands in the same group and cancels, so one group gives `a` and the other gives `b`. The lowest set bit of `x` is `x & -x`.
