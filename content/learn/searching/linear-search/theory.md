## The idea

Linear search is the simplest way to find a value: look at the first position, then the next, and so on, until you meet the target or run out of values. It asks nothing of the data. The values can be in any order, repeat, or arrive one at a time from a file. The price is that, in the worst case, every value gets looked at once.

The sample returns the **first** position that holds the target, because it stops at the first match. When the target is not there at all, it returns `-1`, a position no array has.

## Steps

1. Start at `i = 0`.
2. Compare `a[i]` with `target`.
3. If they are equal, return `i`.
4. Otherwise move on to `i + 1` and go back to step 2.
5. If `i` runs past the last position, return `-1`.

## Cost

| Case | Cost |
|---|---|
| Best | `O(1)` (the target is at position 0) |
| Average | `O(n)`: about `n / 2` checks when the target is somewhere inside |
| Worst | `O(n)` (the target is last, or missing: all `n` values checked) |
| Extra space | `O(1)`: one index |

## Pitfalls

- **Forgetting the missing case.** If the loop ends without a match, the function must still return something. Returning `0` or nothing would look like a real answer; `-1` cannot be confused with a position.
- **Returning too late.** To get the first match, return as soon as you find it. A loop that keeps going and remembers the last `i` that matched returns the last copy instead.
- **Searching the same data again and again.** One linear search is cheap. Thousands of them over the same values cost `O(n)` each; sorting once and using binary search, or building a hash table, pays off quickly.

## When to use it

Use it when the data is small, unsorted, or read only once: a list of a dozen settings, a stream of lines from a file, or a check that runs a single time. It is also the right choice when you need the first match by position, or when the test is more than equality (the first word longer than ten letters, the first failing reading). For repeated lookups in large data, sort and search, or use a hash table.

## Related topics

- [Binary search](learn:searching/binary-search): `O(log n)` checks, once the values are sorted.
- [Big-O growth curves](learn:foundations/big-o-growth): how `n` checks compare with `log n`.
- Hash tables (coming soon): lookups in `O(1)` on average, in any order.
