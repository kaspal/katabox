## Approach: expand around every middle

A piece that reads the same backwards is symmetric around its middle, which is a letter (odd length) or a gap between two neighbours (even length). For each of the 2n − 1 middles, grow outwards while the letters on both sides are equal; the largest growth is the longest mirrored piece with that middle.

Process the middles from left to right (letter 0, gap 0-1, letter 1, gap 1-2, …) and only replace the best piece with a strictly longer one. Among pieces of equal length, a later middle means a later start, so the first-starting one wins.

(Manacher's algorithm finds the same answer in O(n); the Python reference uses it, because expanding in pure Python is slow on long runs of one letter.)

### Walkthrough

"noonmadam": the gap between the two `o`s grows to "noon" (4); the `d` grows to "madam" (5). Nothing longer exists, so "madam".

**Complexity:** O(n²) time in the worst case (a text of one repeated letter), O(1) extra space. Checking every piece separately is O(n³).
