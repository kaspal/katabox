## The idea

**Union-find** (a disjoint-set union) tracks elements split into groups that only ever merge. It answers two operations:

- **union a b**: merge the group of `a` with the group of `b`;
- **find x**: name the group of `x`, so two elements are in the same group exactly when find gives the same answer.

Each group is a tree, stored as one array: `parent[x]` is the element above `x`, and the top of each tree, the **root**, is its own parent and names the group. find climbs parent links to the root; union finds both roots and, if they differ, hangs one under the other.

### Two tricks

Two tricks keep the trees flat:

- **Union by size**: always hang the root of the smaller group under the root of the bigger one (on a tie, `b`'s root goes under `a`'s). A tree's height then grows only when its size at least doubles, so it stays at most `log n`.
- **Path compression**: after find has found the root, it walks the path again and points every node on it straight at the root. The next find from any of them takes one step.

Together they make each operation cost `O(α(n))` amortised, where `α` is the inverse Ackermann function: below 5 for any `n` that fits in a computer, so practically constant.

### The default input

Six elements. union 0 1 and union 2 3 make two pairs, rooted at 0 and 2. union 1 3 joins two groups of size 2, so root 2 goes under root 0. union 4 5 pairs 4 and 5. union 3 5: find 3 climbs 3 → 2 → 0 and compresses, pointing 3 straight at 0; root 4 (size 2) then goes under root 0 (size 4). The output is the roots found by find 5 and find 4, 0 and 0, then the number of groups left: 1.

## Steps

1. Start with `parent[x] = x` and `size[x] = 1` for every element.
2. find `x`: follow parent links up to the root.
3. Then walk from `x` again and set each node's parent to the root.
4. union `a b`: find both roots. If they are equal, do nothing.
5. Otherwise hang the smaller group's root under the bigger one's and add the sizes.

## Cost

| Operation | Cost |
|---|---|
| find, union | `O(α(n))` amortised, practically constant |
| find, union by size only | `O(log n)` |
| Extra space | `O(n)`: the parent and size arrays |

## Pitfalls

- **Linking elements instead of roots.** `parent[a] = b` only moves `a`, not its group, and can break the trees. Always link root to root.
- **Skipping union by size.** Without it, a sequence of unions can build a long chain, and the first find on it is `O(n)`.

## When to use it

Use union-find whenever groups merge and you ask "are these connected?": connected components of a graph built edge by edge, detecting a cycle as edges arrive, and Kruskal (coming soon), which builds a minimum spanning tree.

## Related topics

- [Tree diameter](learn:trees/tree-diameter): another tree stored as parent links.
- [Big-O growth curves](learn:foundations/big-o-growth): how `log n` and nearly constant compare.
- Kruskal (coming soon): union-find picks the edges of a minimum spanning tree.
