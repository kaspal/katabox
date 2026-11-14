## The idea

A **segment tree** answers **range sums** and **point updates** on an array, both in `O(log n)`. A plain array updates fast but sums a range slowly; prefix sums do the opposite. A segment tree sits between the two.

The tree lives in an array of nodes. Node 1 covers the whole range `[0, n − 1]`, and a node `k` that covers `[lo, hi]` has two children: `2k` covers `[lo, mid]` and `2k + 1` covers `[mid + 1, hi]`, where `mid` is `(lo + hi) / 2` rounded down. A node that covers one position is a leaf and holds that value; every other node holds the sum of its two children. Building it touches each node once, `O(n)`, and `4n` slots are always enough.

- **sum l r** starts at the root. A node whose range is fully outside `[l, r]` gives 0. A node fully inside gives its stored sum, without going any deeper. Any other node asks both children and adds their answers. Only a few nodes per level are partly covered, so the walk stays `O(log n)`.
- **update i v** walks from the root down to the leaf of position `i` (left when `i ≤ mid`), sets it to `v`, and on the way back recomputes each node on the path as the sum of its children.

### The default input

Values 5 3 8 6 1 4. The root holds 27. Node 2 covers [0, 2] with 16 and node 3 covers [3, 5] with 11. Below them, node 4 = [0, 1] holds 8, node 5 = [2, 2] holds 8, node 6 = [3, 4] holds 7 and node 7 = [5, 5] holds 4. sum 1 4 takes three nodes fully inside, the leaf [1, 1] (3), node 5 (8) and node 6 (7), and skips the leaves [0, 0] and [5, 5]: the answer is 18. update 2 0 sets node 5 to 0, then recomputes node 2 (now 8) and the root (now 19). sum 0 5 covers the whole array, so the root answers alone: 19. The output is 18, 19.

## Steps

1. Build: a leaf takes its array value; an inner node takes the sum of its two children.
2. sum l r: if a node's range is outside `[l, r]`, return 0.
3. If it is inside `[l, r]`, return the node's sum.
4. Otherwise return the left child's answer plus the right child's.
5. update i v: go down to the leaf of `i`, set it to `v`, and recompute every node on the way back up.

## Cost

| Operation | Cost |
|---|---|
| Build | `O(n)` |
| sum, update | `O(log n)` |
| Extra space | `O(n)`: `4n` node slots |

## Pitfalls

- **Too few slots.** `2n` slots are enough only when `n` is a power of two; with any other `n` the last leaves land past the end. Use `4n`.
- **Mixed bounds.** Here every range is inclusive at both ends, so the outside test is `r < lo` or `hi < l`. Mixing in a half-open range drops or double-counts an end.
- **Skipping the recompute.** After setting the leaf, every ancestor's sum is stale until it is rebuilt from its children.

## When to use it

Use a segment tree when range questions and updates are mixed. It works for any merge that can be split across two halves, such as minimum, maximum or greatest common divisor, not only sums. For sums with point additions, a Fenwick tree is shorter; for updates to whole ranges, add lazy propagation.

## Related topics

- [Prefix sums](learn:foundations/prefix-sums): fast range sums when the array never changes.
- [Fenwick tree](learn:trees/fenwick-tree): prefix sums and point updates in less code.
- [Lazy propagation](learn:trees/lazy-propagation): the same tree with updates to whole ranges.
- [Binary heap and priority queue](learn:trees/binary-heap): another tree stored in an array by index arithmetic.
