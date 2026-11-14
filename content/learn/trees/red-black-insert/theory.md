## The idea

A **red-black tree** is a binary search tree that colours its links to stay balanced. This topic shows the **left-leaning** variant (LLRB), best read as a **2-3 tree** drawn with binary nodes. A 2-3 tree node holds one key or two, and all its leaves sit at the same depth. A **red** link glues a node to its parent so the pair acts as one node with two keys; black links are ordinary edges. Each node stores the colour of the link from its parent. Red links lean left, no node touches two red links, and every path from the root down crosses the same number of black links.

A new key arrives as a red leaf. On the way back up, three checks run at each node, in this order:

1. **Rotate left** if the right child is red and the left is not.
2. **Rotate right** if the left child and its own left child are both red.
3. **Flip colours** if both children are red: they turn black and the node red. In 2-3 terms, a node with three keys splits and its middle key moves up.

A rotation gives the new top the old top's colour and paints the old top red. After each insert the root is painted black.

The classic red-black tree of the CLRS textbook allows red links on either side, so its insert repair tells apart more cases (a red or black uncle, an inner or outer grandchild, and their mirrors). Leaning left cuts them down to these three checks, at the cost of a few extra rotations.

### The default input

10, 20, 30, 40, 50, 25, 5. 20 arrives red on the right of 10, so 10 rotates left: 20 is the root with red 10 on its left. 30 gives 20 two red children, and the flip turns them black. 40 leans right under 30 and is rotated left. 50 gives 40 two red children; the flip makes 40 red, leaning right of the root, so 20 rotates left and 40 becomes the root with red 20 on its left. 25 and 5 arrive as red left children of 30 and 10 and need no fix. The output is the values in pre-order, 40, 20, 10, 5, 30, 25, 50, then their colours, 0, 1, 0, 1, 0, 1, 0.

## Steps

1. Walk down as in a BST (equal keys go right) and attach the key as a new red leaf.
2. Coming back up, at each node: if the right child is red and the left is not, rotate left.
3. Then, if the left child and its left child are red, rotate right.
4. Then, if both children are red, flip their colours and the node's.
5. After each insert, paint the root black.
6. Output the values in pre-order, then their colours (1 red, 0 black).

## Cost

| Operation | Cost |
|---|---|
| insert | `O(log n)`: one path down and back up |
| Rotation or flip | `O(1)` |
| Height | at most about `2 log n` |
| Extra space | `O(n)`: the nodes, one colour bit each |

## Pitfalls

- **Running the checks out of order.** A right rotation creates the two red children the flip then needs.
- **Leaving the root red.** A flip at the root paints it red; repaint it black after every insert.
- **Mixing the variants.** An LLRB never has a red right link; CLRS code allows one, so their fix-ups do not mix.

## When to use it

Red-black trees sit behind many library ordered maps, such as Java's `TreeMap` and most `std::map` implementations, because updates need few rotations. The left-leaning variant is the shortest to write yourself.

## Related topics

- [BST: search, insert, delete](learn:trees/bst-operations): the plain tree the colours keep balanced.
- [B-tree: insert and split](learn:trees/b-tree-insert): the 2-3 tree this variant mirrors.
- [AVL tree: rotations](learn:trees/avl-rotations): the same rotations, guided by heights.
- [Pre-, in- and post-order](learn:trees/tree-traversals): the pre-order walk that writes the output.
