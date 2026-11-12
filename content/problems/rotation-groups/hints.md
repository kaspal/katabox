Comparing every pair of codes is about 2·10⁸ comparisons for 2·10⁴ codes. Instead, can you give every bracelet one standard way of writing it?
---
Choose, among all rotations of a code, the one that comes first in alphabetical order. Two codes are the same bracelet exactly when they have the same chosen rotation.
---
A code of length `L` has `L` rotations, so finding the smallest costs O(L²) with `L ≤ 12`. Put the smallest rotations in a hash set; its size is the answer.
