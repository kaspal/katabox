Trying every triple is O(n³): about 1.3 billion triples for 2000 gifts. Fix the cheapest gift of the trio first; what is left is a question about pairs.
---
For a fixed first gift, look for the pair among the dearer gifts whose sum is closest to `target − first`. In a sorted list, two pointers do that in one sweep.
---
Sort the prices. For each `i`, put `lo = i + 1` and `hi = n − 1`. If the trio sum is below the target, move `lo` right; if above, move `hi` left; if equal, you are done. Keep the best sum, preferring the smaller one on ties.
