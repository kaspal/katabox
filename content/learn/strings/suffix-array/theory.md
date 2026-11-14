## The idea

A **suffix** is the end of a text from some position on. The **suffix array** lists the start positions of all suffixes in alphabetical order. Sorting the suffixes directly compares long pieces of text over and over. **Prefix doubling** sorts them by their first letter, then by their first 2 letters, then 4, then 8, reusing each round's result.

Every suffix gets a **rank**: equal ranks mean "equal so far". At the start the rank is just the first letter. In a round with step `k`, each suffix `i` is sorted by a pair: its own rank, which covers its first `k` letters, and the rank of suffix `i + k`, which covers the next `k`. The two together describe the first `2k` letters. A suffix with nothing at `i + k` uses `-1`, so a shorter suffix comes before a longer one that starts the same way. After sorting, the ranks are renumbered `0, 1, 2…`, giving equal pairs equal ranks. When the last suffix in order has rank `n - 1`, all ranks differ: the order is final. Otherwise `k` doubles. The output is the start positions in sorted order. The text is typed as one word, letters a to z.

### The default input

Text `tatatoma`, 8 suffixes. With `k = 1` the pairs are the first two letters: `ta`, `at`, `ta`, `at`, `to`, `om`, `ma`, and `a` alone. Sorted and renumbered, the ranks are `a` 0, `at` 1 (twice), `ma` 2, `om` 3, `ta` 4 (twice), `to` 5. Two ties remain, so `k` becomes 2. Now suffix 1 (`atatoma`) gets the pair (1, 1) and suffix 3 (`atoma`) gets (1, 3), and suffixes 0 and 2 get (4, 4) and (4, 5). Every rank differs and the loop stops. The order is `a`, `atatoma`, `atoma`, `ma`, `oma`, `tatatoma`, `tatoma`, `toma`, so the output is `[7, 1, 3, 6, 5, 0, 2, 4]`.

## Steps

1. Rank each position by its letter; the order starts as `0, 1, …, n - 1`; `k = 1`.
2. Sort the order by the pair (`rank[i]`, `rank[i + k]` or `-1`).
3. Renumber: walking the sorted order, the rank goes up by one whenever the pair changes.
4. If the last rank is `n - 1`, return the order; otherwise double `k` and go back to step 2.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(n log² n)`: at most `log n` rounds, each a comparison sort |
| Extra space | `O(n)` for the ranks and the order |

After `log n` rounds each pair covers the whole suffix, so the ranks must all differ. Sorting the pairs with a [counting sort](learn:sorting/counting-sort) pass per key instead of a comparison sort brings the total down to `O(n log n)`.

## Pitfalls

- **Missing second half.** A suffix shorter than `2k` has no rank at `i + k`; it needs a value below every real rank, here `-1`.
- **Comparing letters again.** Each round compares two numbers per suffix, never the text; that is the whole saving.
- **Stopping too early.** The loop ends only when all ranks are different, not after a fixed number of rounds.

## When to use it

With the suffixes in order, every occurrence of a pattern sits in one block, which [binary search](learn:searching/binary-search) finds. A suffix shares its longest prefix with one of its neighbours in the array, so the longest repeated substring is the longest prefix two neighbours share. The problem Repeated Chunk, linked from [Rabin-Karp](learn:strings/rabin-karp), asks for exactly that.

## Related topics

- [Merge Sort](learn:sorting/merge-sort): a comparison sort like the one each round uses.
- [Rabin-Karp](learn:strings/rabin-karp): another way to find repeated pieces.
- LCP array (coming soon): the shared prefix of each pair of neighbours.
- Suffix tree (coming soon): a tree of all suffixes, with the same uses.
