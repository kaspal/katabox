## The idea

A **Fenwick tree** (or binary indexed tree) answers **prefix sums** and **point additions** in `O(log n)`, with a single array of `n + 1` numbers and a few lines of code.

Inside the tree, positions count from 1: slot `i + 1` belongs to position `i`, and slot 0 is unused. Let `lowbit(i)` be the lowest set bit of `i`, which `i & -i` computes: lowbit(6) = 2, lowbit(8) = 8, lowbit(5) = 1. Slot `i` stores the sum of the `lowbit(i)` slots that end at `i`. So slot 6 covers slots 5 and 6, slot 8 covers slots 1 to 8, and slot 5 covers only itself.

- **sum i** (positions 0 to `i`) starts at slot `i + 1`, adds it, and jumps to `i − lowbit(i)`, the slot just before the block it added. Each jump clears one set bit, so there are at most `log n` of them.
- **add i v** starts at slot `i + 1` and adds `v`, then jumps to `i + lowbit(i)`, the next slot whose block also contains this one. Each jump moves the lowest set bit higher, so again at most `log n` slots change.

No tree is drawn anywhere: the bit pattern of each index tells it which block it covers and where to jump.

### The default input

n = 8, all zeros. add 1 4 starts at slot 2 and changes slots 2, 4 and 8. add 4 3 starts at slot 5 and changes 5, 6 and 8. add 6 2 starts at slot 7 and changes 7 and 8. Slots 1 to 8 now hold 0 4 0 4 3 3 2 9. sum 5 starts at slot 6 (3) and jumps to slot 4 (4), then to 0, where it stops: 7. sum 7 starts at slot 8, which already covers every position: 9. The output is 7, 9.

## Steps

1. Start with `n + 1` zeros; slot 0 is never used.
2. add i v: set `i` to `i + 1`; while `i ≤ n`, add `v` to slot `i` and set `i` to `i + (i & -i)`.
3. sum i: set `i` to `i + 1` and a total to 0; while `i > 0`, add slot `i` to the total and set `i` to `i − (i & -i)`.
4. For the sum of positions `l` to `r`, take sum `r` minus sum `l − 1`.

## Cost

| Operation | Cost |
|---|---|
| add | `O(log n)` |
| Prefix sum | `O(log n)` |
| Extra space | `O(n)`: `n + 1` slots |

## Pitfalls

- **Using slot 0.** `0 & -0` is 0, so a walk that reaches slot 0 never moves again. That is why the tree counts from 1.
- **Reading a slot as a value.** Slot `i` holds a block sum, not the value at one position. One position's value is sum `i` minus sum `i − 1`.
- **Asking for a minimum.** A range sum works as a difference of two prefix sums; a range minimum or maximum cannot be undone that way. Use a segment tree for those.

## When to use it

Use a Fenwick tree for running totals that keep changing: counting inversions, frequency tables, the rank of a value among those seen so far. It is shorter and faster in practice than a segment tree; switch to a segment tree when you need minimums, maximums or range updates.

## Related topics

- [Prefix sums](learn:foundations/prefix-sums): the same answers when the array never changes.
- [Segment tree](learn:trees/segment-tree): more general range questions, with more code.
- [Lazy propagation](learn:trees/lazy-propagation): range updates on a segment tree.
- [Bit manipulation basics](learn:foundations/bit-manipulation): why `i & -i` isolates the lowest set bit.
