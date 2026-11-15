## Approach: row heights and the largest rectangle under a skyline

Every banner has a bottom edge on some row. Fix that row and, for each column `x`, let `height[x]` be the number of free squares in the unbroken run that ends on this row in column `x`. A banner with its bottom edge here, spanning columns `left..right`, can be at most `min(height[left..right])` tall. So the best banner on this row is the **largest rectangle under the skyline** of the heights, and the answer is the best over all rows.

The heights move from one row to the next in O(C): a free square adds 1, a blocked one resets to 0. For Example 1:

```
.#...   1 0 1 1 1
.....   2 1 2 2 2
##...   0 0 3 3 3   → 3 bars of height 3: area 9
.#..#   1 0 4 4 0
```

**The skyline with a stack.** For each bar, the widest rectangle of exactly its height reaches from just after the nearest lower bar on the left to just before the nearest lower bar on the right. Sweep the columns left to right, keeping a stack of columns with increasing heights, and add a column of height 0 at the end:

1. Before pushing column `x`, pop every column whose height is at least `height[x]`.
2. A popped column of height `h` reaches right up to `x − 1` (column `x` is the first one lower or equal) and left to just after the column now on top of the stack (everything between was taller and has already been popped), or to column 0 if the stack is empty. Its area is `h · (x − left)`.
3. Push `x`.

**Why it is correct:** the largest rectangle under the skyline has the height of its lowest bar, and it cannot extend past a lower bar on either side; so it is the widest rectangle of some bar's height, and the sweep computes that width for every bar. With equal heights, the first of two equal bars is popped early with a shorter width, but the last bar of the run is popped later and gets the full width, so no area is lost. Every column is pushed and popped once, so a row costs O(C).

**The obvious approach is too slow.** Trying every left edge and widening to the right while tracking the lowest bar costs O(C²) per row, O(R · C²) in all: on a 1500 × 1500 wall with only a few dozen squares blocked, almost every row is one long free run and that is about 1.7·10⁹ steps. Checking every rectangle square by square is far worse. The stack makes the whole wall O(R · C), about 10⁶ steps.

**Complexity:** O(R · C) time and O(C) memory beyond the input.

**Practical notes:** read all the input at once and split it on whitespace; the rows contain no spaces, so each row is one token. In TypeScript and Rust, compare character codes (`charCodeAt`, `as_bytes()`) rather than building one-character strings. The area is at most 10⁶, so any integer type is enough.
