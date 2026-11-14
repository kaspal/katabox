## The idea

Two words share a **common subsequence** when the same letters appear in both, in the same order, with anything in between. Which shared subsequence is the longest? Listing every subsequence of one word and checking it against the other doubles the work with each letter. The table instead compares **prefixes**.

`best[i][j]` is the length of the longest common subsequence of the first `i` letters of word `a` and the first `j` letters of word `b`. Row 0 and column 0 stand for an **empty prefix**, so they are all zeros. For each other cell, look at the last letters, `a[i - 1]` and `b[j - 1]`:

- If they **match**, that letter can close a shared subsequence: `best[i - 1][j - 1] + 1`, the **diagonal** cell plus one.
- If not, one of the two letters is not used. Drop the last letter of `a` (the cell **above**) or of `b` (the cell to the **left**), and keep the larger.

The answer is the bottom-right cell. A **traceback** from there recovers the letters: on a match, record the letter and step diagonally; otherwise step toward the larger neighbour, **up** on a tie. The letters come out backwards, so the sample reverses them. The output is the length, then the letters as numbers 0 to 25 (a = 0), which the app shows as letters. The words are typed in two fields, letters a to z.

### The default input

Words `planet` and `anklet`. The row for `p` is all zeros: no match. The row for `l` reaches 1 at the `l` of `anklet`. The row for `a` is 1 from its first column on. The row for `n` matches at column 2 with `1 + 1 = 2`. The row for `e` matches at column 5 with `2 + 1 = 3`. The row for `t` matches in the corner with `3 + 1 = 4`.

The traceback takes `t` and `e` diagonally, steps left twice past `l` and `k` (the left cell holds 2, the one above 1), then takes `n` and `a`. The output is `[4, 0, 13, 4, 19]`: the letters `ante`.

## Steps

1. Make a table of `m + 1` rows and `n + 1` columns, all zeros.
2. Fill row by row: on a match, diagonal + 1; otherwise the larger of up and left.
3. Traceback from `(m, n)`: on a match, record the letter and go diagonal; else go up if `up ≥ left`, otherwise left.
4. Return the length and the recorded letters, reversed.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(m · n)`: one cell per pair of prefixes |
| Extra space | `O(m · n)` for the table, which the traceback needs |

## Pitfalls

- **Off by one.** Cell `(i, j)` compares `a[i - 1]` with `b[j - 1]`; the extra row and column are what make the empty prefix work.
- **Substring is not subsequence.** A common **substring** must be contiguous and needs a different rule: on a mismatch the cell resets to 0.
- **More than one answer.** Several subsequences can share the longest length; the tie rule decides which one the traceback finds.

## When to use it

Use it to compare two sequences: file differences, shared melodies, DNA fragments. The linked problem, Shared Melody, is this table on lists of notes.

## Related topics

- [Edit distance](learn:dynamic-programming/edit-distance): the same grid of prefixes, counting fixes instead.
- [Longest increasing subsequence](learn:dynamic-programming/lis): one sequence, rising values.
- [0/1 knapsack](learn:dynamic-programming/knapsack-01): another two-dimensional table with a traceback.
- [Fibonacci: memo and table](learn:dynamic-programming/fibonacci-memo-table): where tables of subproblems start.
