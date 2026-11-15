## Approach: at most K minus at most K − 1, with a sliding window

Counting stretches with **exactly** `K` sources directly is awkward: as a stretch grows, it can pass through `K` and go beyond. Counting stretches with **at most** `m` sources is easy, and

```
exactly K  =  (at most K)  −  (at most K − 1)
```

because a stretch with at most `K` sources has either exactly `K` or at most `K − 1`.

**At most m, with a window.** If a stretch has at most `m` different sources, every stretch inside it does too. So for a fixed end `right`, the good starts are `left..right` for some smallest start `left`, and when `right` moves on, `left` can only stay or move right. Keep a hash map from id to its count inside the window:

1. Add `ids[right]` to the map.
2. While the map holds more than `m` ids, take `ids[left]` out (delete the id when its count reaches 0) and move `left` on.
3. Every stretch from `left..right` to `right` qualifies: add `right − left + 1`.

With `m = 0` the answer is 0 (no stretch is empty), which covers `K = 1`.

**Worked example** (`4 4 7 4 9 9`, `K = 2`). At most 2: the window adds 1, 2, 3, 4, then `9` brings a third source, so the window shrinks to `4 9` and adds 2, then 3: 15 in all. At most 1: 1, 2, 1, 1, 1, 2: 8. The answer is 15 − 8 = 7.

**Why it is correct:** step 2 removes exactly the starts that would leave more than `m` sources, and no start before `left` can become good again later, because a longer stretch never has fewer sources. So step 3 counts every stretch with at most `m` sources once, at its end.

**Complexity:** each second enters and leaves the window at most once per pass, so O(n) expected time with a hash map, and O(n) memory.

**Why the obvious way is too slow:** extending from every start until it has more than `K` sources is O(n²). With `n = 10⁵` and a recording where most stretches stay at `K` sources to the end, that is about 5·10⁹ steps.

**Practical notes:** read all tokens at once and print the single answer. The ids go up to 10⁹, so count them in a hash map (`dict`, `Map`, a PHP array with integer keys, `HashMap`), not in an array indexed by id. The count can reach `n(n + 1)/2 ≈ 5·10⁹`, which overflows 32 bits: use `u64` or `i64` in Rust (it is still exact in a TypeScript `Number`).
