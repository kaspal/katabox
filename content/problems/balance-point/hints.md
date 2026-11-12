Adding up both sides for every candidate index works, but each candidate costs a full pass: O(n²) in total.
---
If you know the total of the whole row and the total to the left of index `i`, what is the total to its right?
---
Compute `total` once. Walk left to right keeping `left` (the sum before `i`): the right side is `total - left - weights[i]`. Return the first `i` where they are equal, then add `weights[i]` to `left` and go on.
