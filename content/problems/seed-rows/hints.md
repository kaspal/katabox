Listing plantings one by one is hopeless: there can be astronomically many. Count them by how the planting **ends**: which kind is the last plant, and how much of the bed it fills.
---
Let `ways[x][k]` be the number of valid plantings of the first `x` centimetres whose last plant is of kind `k`. Removing that last plant leaves a valid planting of `x − widths[k]` centimetres that does **not** end with kind `k` (or the empty bed).
---
So `ways[x][k] = total[x − w] − ways[x − w][k]` with `w = widths[k]`, where `total[y]` is the sum over all kinds and `total[0] = 1` (the empty bed). The answer is `total[length]`, all modulo 1 000 000 007.
