## The idea

How many single-letter fixes turn word `a` into word `b`? A fix is one of three edits, each costing 1: **insert** a letter, **delete** a letter, or **replace** one letter with another. The fewest fixes is the **edit distance** (also called Levenshtein distance). Spell checkers rank their suggestions by it.

`cost[i][j]` is the fewest fixes that turn the first `i` letters of `a` into the first `j` letters of `b`. The edges are easy: turning `i` letters into nothing takes `i` deletions, so column 0 holds `i`; building `j` letters from nothing takes `j` insertions, so row 0 holds `j`.

For every other cell, look at the last letters, `a[i - 1]` and `b[j - 1]`. If they are the **same**, they need no fix: copy the **diagonal**, `cost[i - 1][j - 1]`. If they differ, the last fix is one of three, and each neighbour stands for one:

- **Diagonal**, `cost[i - 1][j - 1]`: **replace** `a[i - 1]` with `b[j - 1]`, after turning the shorter prefixes into each other.
- **Up**, `cost[i - 1][j]`: **delete** `a[i - 1]`; the rest of `a` already became all `j` letters.
- **Left**, `cost[i][j - 1]`: **insert** `b[j - 1]` at the end; all `i` letters already became the first `j - 1`.

The cell takes the smallest of the three, plus 1. The output is the corner, `[cost[m][n]]`.

### The default input

Words `carpet` and `carrot`. Row 0 runs 0 to 6, and so does column 0. The first three letters match, so the diagonal stays at 0 through `cost[3][3]`. At `cost[4][4]`, `p` meets `r`: the diagonal offers 0, so the cell is 1, one replacement. At `cost[5][5]`, `e` meets `o`: the diagonal offers 1, and the cell is 2. The last letters are both `t`, so the corner copies the diagonal: 2.

The output is `[2]`: replace `p` with `r` and `e` with `o`.

## Steps

1. Make a table of `m + 1` rows and `n + 1` columns; set `cost[i][0] = i` and `cost[0][j] = j`.
2. Fill row by row: if `a[i - 1] = b[j - 1]`, copy the diagonal; otherwise take 1 + the smallest of diagonal, up and left.
3. Return `cost[m][n]`.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(m · n)`: one cell per pair of prefixes |
| Extra space | `O(m · n)` for the table |

## Pitfalls

- **Forgetting the edges.** With zeros in row 0 and column 0, inserting or deleting at the start would be free.
- **Adding 1 to a match.** Equal letters copy the diagonal; the 1 is only for a real fix.
- **Swapped letters.** Two neighbouring letters in the wrong order cost two replacements here, as in the "swapped letters" preset (`form` and `from`). Counting a swap as one fix is a different distance.

## When to use it

Use it to measure how far apart two strings are: typo suggestions, fuzzy search, comparing short sequences. The linked problem, Typo Distance, is exactly this.

## Related topics

- [Longest common subsequence](learn:dynamic-programming/lcs): the same grid of prefixes, keeping letters instead of fixing them.
- [Paths through a grid](learn:dynamic-programming/grid-paths): a table filled from the up and left neighbours.
- [0/1 knapsack](learn:dynamic-programming/knapsack-01): another two-dimensional table.
- [Rolling hash](learn:hashing/rolling-hash): fast exact matching, when no fixes are allowed.
