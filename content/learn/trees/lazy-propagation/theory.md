## The idea

A segment tree changes one position in `O(log n)`, but adding a value to a whole range one position at a time costs `O(n log n)`. **Lazy propagation** makes range updates `O(log n)` as well, by putting work off until it is needed.

Each node keeps the **sum** of its range and a **tag**: an amount already added to its whole range but not yet passed on to its children. The array starts at zeros.

- **add l r v** walks down like a range sum. A node fully outside `[l, r]` is skipped. A node fully inside is updated on the spot: its sum grows by `v` times its length, its tag by `v`, and the walk stops there. A partly covered node first **pushes** its tag to both children (each adds it to its own tag, and the tag times its length to its sum) and resets it to 0; then the walk enters both children and recomputes the node's sum from theirs.
- **sum l r** is the ordinary range sum, except that it pushes a partly covered node's tag before going below it.

So tags move down only when a later operation needs to go below them.

### The default input

n = 8. add 0 5 2 takes two nodes whole: it tags [0, 3] (its sum grows by 8) and [4, 5] (by 4), and the root's sum becomes 12. add 3 7 1 must go inside [0, 3], so it pushes that tag of 2 to [0, 1] and [2, 3]; inside [2, 3] it pushes again, to [2, 2] and [3, 3]. Then [3, 3] takes the +1 (sum 3) and [4, 7] is tagged +1, its sum growing by 4 to 8. The array is now 2 2 2 3 3 3 1 1. sum 2 6 reads [2, 3] directly (5), pushes [4, 7]'s tag to read [4, 5] (6), then pushes [6, 7]'s tag to read [6, 6] (1): 12. sum 0 0 pushes [0, 1]'s tag and reads 2. The output is 12, 2.

## Steps

1. To apply `v` to a node: add `v` times its length to its sum and `v` to its tag.
2. To push a node: if its tag is not 0, apply the tag to both children and set it to 0.
3. add l r v: skip nodes outside, apply `v` to nodes inside, and for any other node push, recurse into both children and recompute its sum.
4. sum l r: give 0 outside and the stored sum inside; for any other node push, then add both children's answers.

## Cost

| Operation | Cost |
|---|---|
| add on a range | `O(log n)` |
| sum on a range | `O(log n)` |
| Extra space | `O(n)`: sums and tags, `4n` slots each |

## Pitfalls

- **Reading children before pushing.** A child below a tagged node misses that tag; always push before going deeper.
- **Forgetting the length.** A tag counts per position. When it is applied, a node's sum grows by the tag times the number of positions it covers, not by the tag alone.
- **Skipping the recompute in add.** After the children change, the node's sum must be rebuilt from them, or later sums that stop at this node will be wrong.

## When to use it

Use lazy propagation when whole ranges change and range questions keep coming: adding to a stretch of days or shifting all prices in a range. Other kinds of tag (such as "set to x") work the same way once you know how two tags combine and how a tag changes a sum.

## Related topics

- [Segment tree](learn:trees/segment-tree): the tree this builds on, with point updates.
- [Fenwick tree](learn:trees/fenwick-tree): a lighter structure for prefix sums and point additions.
- [Prefix sums](learn:foundations/prefix-sums): range sums when the array never changes.
