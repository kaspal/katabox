## The idea

The **lowest common ancestor** (LCA) of two nodes in a rooted tree is the deepest node that has both of them below it, where a node counts as its own ancestor. Climbing one parent at a time costs `O(depth)` per question. **Binary lifting** prepares jumps of 1, 2, 4, 8, ... levels once, so that every question takes `O(log n)`.

The table: `up[0][v]` is the parent of `v`, and the root 0 is its own parent, so a jump past the top stops at the root. Then `up[k][v] = up[k−1][up[k−1][v]]`: a jump of `2^k` levels is two jumps of `2^(k−1)`. The table has about `log n` rows. Each node's depth is its parent's plus 1; every parent is numbered lower than its child, so one pass in order fills them.

A question has two phases. First lift the deeper node by the depth difference, one table jump per set bit (a difference of 5 = 4 + 1 is two jumps). Then, if the nodes still differ, raise both together, from the longest jump down, but only when the jump keeps them apart: a jump that lands them together might pass the LCA. They finish as two different children of the LCA, whose parent is the answer.

### The default input

Parents 0 0 1 1 2 2 3 4 6, for nodes 1 to 9: nodes 1 and 2 hang under the root, 3 and 4 under 1, 5 and 6 under 2, and 7, 8 and 9 under 3, 4 and 6. Ten nodes give 4 rows. Pair (7, 8): both have depth 3. Their ancestors 2 levels up are both 1, so that jump is skipped; their parents, 3 and 4, differ, so both step up once, and the answer is the parent of 3: 1. Pair (5, 9): 9 is one level deeper and lifts to 6. 6 and 5 differ, but every jump from them lands on the same node, so nothing moves, and the answer is the parent of 6: 2. Pair (3, 4): their parents are already the same, so the answer is 1. The output is 1, 2, 1.

## Steps

1. Fill row 0 with the parents (the root points at itself), and each depth as the parent's depth plus 1.
2. Fill row `k` from row `k − 1`: `up[k][v] = up[k−1][up[k−1][v]]`.
3. For a pair, make `a` the deeper node and lift it by the depth difference, one jump per set bit.
4. If `a` equals `b`, it is the answer.
5. Otherwise, for `k` from the top row down to 0, if `up[k][a]` differs from `up[k][b]`, move both up by `2^k`.
6. The answer is `up[0][a]`.

## Cost

| Operation | Cost |
|---|---|
| Build the table | `O(n log n)` |
| One pair | `O(log n)` |
| Extra space | `O(n log n)` for the table |

## Pitfalls

- **Lifting the wrong node.** Swap first so `a` is the deeper one; lifting the shallower node by the difference overshoots.
- **Jumping when the ancestors match.** Equal ancestors may be the LCA or above it. Jump only while they differ, and take the parent at the end.
- **Too few rows.** The table needs enough rows that the longest jump covers the deepest node; `⌊log₂ n⌋ + 1` rows always suffice.

## When to use it

Use binary lifting when a fixed tree gets many ancestor questions: the distance between two nodes (depth of `a` plus depth of `b` minus twice the depth of their LCA), or the ancestor `k` levels up. If the tree changes, the table must be rebuilt.

## Related topics

- [Tree diameter](learn:trees/tree-diameter): another tree stored through parent links, measured by depths.
- [Bit manipulation basics](learn:foundations/bit-manipulation): reading the set bits of the depth difference.
- [Union-find](learn:trees/union-find): parent arrays again, used for merging groups.
