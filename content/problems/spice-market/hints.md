Because any part of a sack can be taken, every gram of the bag is filled independently. Which gram of spice is the best one to put in?
---
Compare spices by their value per gram, `prices[i] / weights[i]`. A gram of the most valuable spice per gram is always at least as good as a gram of any other.
---
Sort the stalls by value per gram, highest first, and take whole sacks while they fit. Fill what is left with part of the next sack and stop. To compare without rounding, compare `p1 · w2` with `p2 · w1`.
