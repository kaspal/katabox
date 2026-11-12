## Approach: one total, one running sum

The right side of index `i` is everything except the left side and the crate itself:

`right = total - left - weights[i]`

So compute `total` in one pass, then walk the row once more with `left = 0`. At each index compare `left` with `total - left - weights[i]`; the first match is the answer. After the check, add `weights[i]` to `left`.

### Walkthrough

`weights = [4, -1, 6, 2, 1]`, `total = 12`:

| i | left | right = 12 − left − w | balanced? |
| --- | --- | --- | --- |
| 0 | 0 | 8 | no |
| 1 | 4 | 9 | no |
| 2 | 3 | 3 | **yes** |

**Complexity:** O(n) time, O(1) extra space. Summing both sides from scratch for every index is O(n²), too slow for 10⁵ crates.
