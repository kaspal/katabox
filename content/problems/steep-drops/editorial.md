## Approach: merge sort with a counting pass

Every steep drop `(i, j)` either has both days in the first half, both in the second half, or `i` in the first half and `j` in the second. The first two kinds are counted recursively. For the third kind, the order of days **inside** each half no longer matters, only that the first-half day comes earlier, so both halves can be sorted.

With `left` and `right` sorted ascending, move a pointer `j` through `right` while `x > 2 · right[j]`, for each `x` of `left` in order. Because `x` only grows, `j` never moves back, and `j` is the number of steep drops ending in the right half that start at `x`. That is O(length) per merge step.

Then merge `left` and `right` into one sorted list for the caller.

Note the difference from counting inversions: there the counting can happen during the merge itself, but here the condition `x > 2y` does not match the merge's `x > y`, so the counting needs its own pass.

### Walkthrough

`[9, 4, 3, 10, 2]` splits into `[9, 4]` and `[3, 10, 2]`. Inside: (9, 4) is a drop; in the right part, (10, 2). Across, with sorted `[4, 9]` and `[2, 3, 10]`: 4 is not > 2·2, so 0; 9 > 4 and 9 > 6 but not > 20, so 2. Total 1 + 1 + 2 = 4.

**Complexity:** O(n log n) time, O(n) extra space. Checking every pair is O(n²), 2·10¹⁰ checks for 2·10⁵ days.
