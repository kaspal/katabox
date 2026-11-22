## Idea

The base problem compared the letter counts once. Here keep `diff[letter] = count in a − count in b` and `off`, the number of letters with `diff ≠ 0`; the bags are twins exactly when `off == 0`. A change at position `p` from `old` to `new` takes one `old` out of `b` (`diff[old] += 1`) and puts one `new` in (`diff[new] −= 1`). For each of the two updates, note whether that difference was zero before and is zero after, and correct `off` accordingly. Store `b` as a mutable array to know `old`. If the bags have different sizes, the differences cannot all be zero, so the answer is always `false` with no special case needed.

## Complexity

O(|a| + |b|) to build the counts, then O(1) per change: O(|a| + |b| + q) time in total. O(|b|) extra memory for the copy of `b`, plus 26 counters.
