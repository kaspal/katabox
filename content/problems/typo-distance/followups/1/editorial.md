## Idea

Use the base problem's recurrence, but only where it can matter. The cell `(i, j)` (the
distance between the first `i` letters of `typed` and the first `j` of `meant`) is at
least `|i − j|`, because that many inserts or deletes are needed just to match the
lengths. So a cell with `|i − j| > k` is already "more than `k`", and any path through it
ends above `k`.

If `|typed.length − meant.length| > k`, return `false`. Otherwise fill each row only for
`j` from `i − k` to `i + k`, read the cells just outside the band as `k + 1`, and cap
every value at `k + 1`. The capped table still says correctly whether each cell is at
most `k`, so the answer is whether the final cell is at most `k`.

## Complexity

O(typed.length · k) time, about 2·10⁴ · 41 ≈ 8·10⁵ cells, and O(k) extra memory when you
keep only the previous row's band.
