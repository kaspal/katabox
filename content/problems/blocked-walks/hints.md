Without roadworks, a walk is a choice of which `rows` of its `rows + cols` steps go south: C(rows + cols, rows) walks. A grid DP over all crossings would take 10¹⁰ steps, so count walks with formulas instead.
---
Count the bad walks by the **first** closed crossing they hit. If `good[i]` is the number of walks from the start to closed crossing `i` that avoid every other closed crossing, the walks whose first closed crossing is `i` number `good[i] · paths(i → end)`.
---
Sort the closed crossings (by row, then column) and add the destination at the end. `good[i] = paths(start → i) − Σ good[j] · paths(j → i)` over the earlier crossings `j` that can reach `i`. The answer is `good[destination]`. Use factorial tables and modular inverses for `paths`.
