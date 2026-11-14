## The idea

A **B-tree** keeps several sorted keys in each node, and a node with `k` keys has `k + 1` children, one for each gap between and around its keys. This topic allows at most **2 keys per node**, which makes it a **2-3 tree**. The B-trees inside databases and file systems hold hundreds of keys per node, sized to one disk page, so a lookup reads only a few pages even among billions of keys.

insert walks down from the root, at each node entering the child for the gap the key falls into (equal keys go right), until it reaches a leaf, and puts the key into that leaf in sorted order. If the leaf now holds **3 keys**, it **splits**: the middle key moves up into the parent, and the keys on either side become two separate nodes. The parent may now hold 3 keys and split in turn, and so on up the path. When the root splits, its middle key becomes a **new root** with one key and two children.

So the tree grows only at the top, and all leaves always sit at the same depth.

### The default input

10, 20, 30, 40, 50, 60, 70, 25. 30 turns the first leaf into 10 20 30, which splits: 20 becomes a new root over the leaves 10 and 30. 50 makes 30 40 50, which splits and sends 40 up, so the root holds 20 40 over the leaves 10, 30 and 50. 70 makes 50 60 70 split and send 60 up; the root, now 20 40 60, splits too, and 40 is the new root over 20 (leaves 10 and 30) and 60 (leaves 50 and 70). Last, 25 walks 40 → 20 → the leaf 30 and joins it as 25 30. The output lists the nodes level by level, each followed by a 0: 40 0, then 20 0 60 0, then 10 0 25 30 0 50 0 70 0.

## Steps

1. Start with one empty leaf as the root.
2. At each inner node, step past every key that is not larger than the new key and enter that child.
3. At the leaf, insert the key in sorted order.
4. If a node holds 3 keys, split it: its first and last keys become two nodes, and the middle key goes into the parent between them.
5. If the root splits, make a new root holding only the middle key.
6. Output the nodes level by level, left to right, each followed by 0.

## Cost

| Operation | Cost |
|---|---|
| insert | `O(log n)`: one path down, splits on the way back |
| One split | `O(1)` here; `O(m)` with `m` keys per node |
| Height | between about `log₃ n` (full nodes) and `log₂ n` |
| Extra space | `O(n)` for the nodes |

## Pitfalls

- **Forgetting the children in a split.** An inner node that splits also divides its children: the left part keeps those left of the middle key, the right part the rest.
- **Putting a new key into an inner node.** New keys always go into a leaf; an inner node only gains a key that a split pushed up.
- **Growing the tree at the bottom.** A new leaf below a full one breaks the equal depth of the leaves; split instead.

## When to use it

Use a B-tree when data lives on disk or memory access is the bottleneck: database indexes and file systems read one node per page. Wide nodes also suit the cache, which is why Rust's `BTreeMap` is a B-tree. The 2-3 tree here is the smallest case, best for watching the splits.

## Related topics

- [BST: search, insert, delete](learn:trees/bst-operations): the one-key-per-node tree a B-tree generalises.
- [Red-black tree: insert](learn:trees/red-black-insert): a 2-3 tree drawn with binary nodes and red links.
- [AVL tree: rotations](learn:trees/avl-rotations): balance by rotations instead of splits.
- [Level order](learn:trees/level-order): the row-by-row walk that writes the output.
