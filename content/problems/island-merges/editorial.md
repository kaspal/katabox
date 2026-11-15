## Approach: union-find with an island counter

The obvious method fills a square, then recounts the islands with a flood fill over all the land. With 5·10⁴ squares scattered over a 1000 × 1000 sea, the land grows to 5·10⁴ squares and the recounts visit about 5·10⁴ · 5·10⁴ / 2 ≈ 1.25·10⁹ squares in all. Far too slow, and wasteful: one new square only affects its four neighbours.

**How one square changes the count.** If the square is already land, nothing changes. Otherwise it is first an island of its own: `islands += 1`. Then look at its four side neighbours. Each neighbour on an island the new square has not joined yet merges with it: `islands −= 1`. Two neighbours on the same island must count only once, so the question is always "are these two squares on the same island already?".

**Union-find** answers that. Every land square has a `parent`; following parents leads to the island's root, and two squares are on the same island exactly when they have the same root. Merging two islands links one root to the other.

Example 1, fourth square (1, 2): it starts as a fourth island (4). Its neighbour (1, 1) has a different root: merge, 3. (1, 3): different again, 2. (2, 2): different, 1. Row 0 above is sea. The answer is 1.

**Layout.** Number the square in row `r`, column `c` as `r·(C + 2) + c`, and leave a border of sea around the map (rows 0 and `R + 1`, columns 0 and `C + 1`). The neighbours are then `cell ± 1` and `cell ± (C + 2)`, never off the map, with no bounds checks. One flat array `parent` of `(R + 2)·(C + 2)` entries holds −1 for sea and the parent for land.

**Why it is correct:** the islands are the connected groups of land squares. Adding a square adds one group, then joins it with every distinct group it touches by a side; each join lowers the count by exactly one, and touching the same group twice is caught by the root comparison. Corners are never examined, so diagonal squares stay apart.

**Complexity:** O(R·C) to set up the array, then at most four root searches per square, each O(log(R·C)) amortised at worst with path halving and close to constant in practice. Memory O(R·C), about 10⁶ entries.

**Practical notes:** read all tokens at once and write all `Q` answers in one go. Find roots with a loop (`parent[x] = parent[parent[x]]; x = parent[x]`), not recursion: a chain can be thousands of squares long, beyond Python's recursion limit. In TypeScript an `Int32Array` keeps the million entries compact. In PHP, pass the array to a helper function by reference (`array &$parent`): passing it by value copies it on the first write.
