Let `best[i]` be the longest rising pick that ends at tower `i`. How does it relate to the towers before `i`? That gives an O(n²) solution, which is too slow here.
---
Instead, for each length `k`, keep the smallest possible height of the last tower of a rising pick of length `k`. Why is this list itself strictly increasing?
---
Scan the towers. For height `h`, find the first entry in that list that is `≥ h` (binary search) and replace it with `h`; if there is none, append `h`. The final length of the list is the answer.
