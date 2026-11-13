## Approach: binary search, using the sorted half

Search within `[lo, hi]`, starting with the whole shelf. Look at the middle `m`:

- if `shelf[m] = label`, done;
- if `shelf[lo] ≤ shelf[m]`, the left half `lo..m` is sorted (the break between the two runs is not in it). If `shelf[lo] ≤ label < shelf[m]`, the label can only be there: `hi = m − 1`; otherwise `lo = m + 1`;
- otherwise the right half `m..hi` is sorted. If `shelf[m] < label ≤ shelf[hi]`, search it: `lo = m + 1`; otherwise `hi = m − 1`.

When `lo > hi`, the label is not on the shelf.

### Walkthrough

`[41, 57, 63, 8, 15, 22]`, label 15: m = 2 (63); the left half 41..63 is sorted and 15 is not in [41, 63), so lo = 3. m = 4 (15): found.

**Complexity:** O(log n) time, O(1) space.
