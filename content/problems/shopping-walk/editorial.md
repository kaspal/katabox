## Approach: a sliding window with a "missing" counter

Count the list: `want[c]` is how many stalls of letter `c` you need, and `missing = need.length`.

Move the right end `r` over the stalls. Adding stall `c = stalls[r]`: if `have[c] < want[c]`, it covers one more item, so `missing--`; then `have[c]++`.

While `missing = 0`, the window `[l, r]` covers the list: record `r − l + 1`, then remove stall `l`: `have[c]--`, and if that made `have[c] < want[c]`, `missing++`; move `l` on.

Every shortest window is found: when `r` reaches its right end, the left end moves exactly up to its left end before the window stops covering.

### Walkthrough

`stalls = "bcaacbdab"`, `need = "aab"`: the window first covers at r = 3 ("bcaa", length 4); dropping the `b` breaks it. At r = 5 it covers again as "caacb" and shrinks to "aacb" (length 4) before dropping an `a` breaks it. At r = 7 it covers as "acbda" (length 5). The best is 4.

**Complexity:** O(n + m) time, O(1) extra space (26 counters). Growing a window from every start is O(n²).
