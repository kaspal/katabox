## The idea

Given sorted values and a `target`, find two positions whose values add up to it. Checking every pair costs `O(n²)`. Two pointers do it in one pass: `i` starts at the smallest value, `j` at the largest, and you look at `a[i] + a[j]`.

- If the sum is the target, you are done.
- If it is **too small**, `a[i]` is useless: even paired with the largest value still in play, `a[j]`, it falls short, and every other partner is smaller. So `i` moves right.
- If it is **too big**, `a[j]` is useless for the mirror reason: even the smallest partner left, `a[i]`, overshoots. So `j` moves left.

### Why no pair is skipped

Each move drops a value that cannot belong to any valid pair with the values still between the pointers. So if a valid pair exists, both of its positions stay inside `i..j` until the pointers land on exactly them. When `i` and `j` meet, every value has been ruled out, and the answer is `[-1, -1]`.

## Steps

1. Set `i = 0` and `j = len(a) - 1`.
2. While `i < j`, compute `total = a[i] + a[j]`.
3. If `total == target`, return `[i, j]`.
4. If `total < target`, move `i` one place right.
5. Otherwise move `j` one place left.
6. When the pointers meet, return `[-1, -1]`.

## Cost

| Case | Cost |
|---|---|
| Best | `O(1)` (the outermost pair is the answer) |
| Average | `O(n)` |
| Worst | `O(n)`: each step moves one pointer, at most `n − 1` steps |
| Extra space | `O(1)`: two indices |

If the values are not sorted yet, sorting first adds `O(n log n)`.

## Pitfalls

- **Unsorted input.** The whole argument rests on "moving right makes the sum bigger". Without order, moving a pointer can skip the answer.
- **Using `i <= j`.** When the pointers meet they point at the same value, and a value cannot pair with itself. Stop at `i < j`.
- **Sorting loses the original positions.** If the task wants positions in the original, unsorted list, sort pairs of (value, position), or use a hash table instead.

## When to use it

Use it for pair questions on sorted data: two values with a given sum or difference, the pair closest to a target, or three values with a given sum (fix one, run two pointers on the rest, `O(n²)` in all). The same "one pointer per end, move the one that cannot help" shape also merges sorted lists and checks palindromes.

## Related topics

- [Sliding window](learn:searching/sliding-window): two pointers that both move forward.
- [Binary search](learn:searching/binary-search): pairs in `O(n log n)` by searching for `target − a[i]`.
- [Merge Sort](learn:sorting/merge-sort): its merge walks two pointers through two runs.
- Hash tables (coming soon): pair sums in `O(n)` without sorting.
