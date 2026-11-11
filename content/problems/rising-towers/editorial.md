## Approach: smallest tails and binary search

**The O(n²) idea.** Let `best[i]` be the longest rising pick ending at tower `i`: one more than the largest `best[j]` with `j < i` and `heights[j] < heights[i]` (or 1). The answer is the largest `best[i]`. With 5·10⁴ towers that is about 1.25·10⁹ comparisons: too slow.

**Faster.** Keep `tails`, where `tails[k]` is the smallest height that can end a rising pick of length `k + 1` among the towers seen so far. `tails` is strictly increasing: a pick of length `k + 2` contains a pick of length `k + 1` whose last tower is lower.

For each height `h`, find the first position `p` with `tails[p] ≥ h` (binary search, because `tails` is sorted):

- if there is none, `h` extends the longest pick: append it;
- otherwise `h` is a lower end for picks of length `p + 1`: set `tails[p] = h`.

Using "≥ h" (not "> h") is what makes the rise strict: an equal height replaces, it never extends. The length of `tails` at the end is the answer. (`tails` is not itself a valid pick, only its length matters.)

### Walkthrough

`heights = [5, 1, 8, 3, 6, 9, 4]`:

| h | tails after |
| --- | --- |
| 5 | [5] |
| 1 | [1] |
| 8 | [1, 8] |
| 3 | [1, 3] |
| 6 | [1, 3, 6] |
| 9 | [1, 3, 6, 9] |
| 4 | [1, 3, 4, 9] |

The length is 4.

**Complexity:** O(n log n) time, O(n) extra space.
