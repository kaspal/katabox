## The idea

A **binary search tree** (BST) keeps keys in nodes so that, at every node, the keys in the left subtree are smaller and the keys in the right subtree are not. That one rule lets each operation walk a single path from the root instead of looking at every node. insert walks down (left when the key is smaller, right otherwise) and attaches a new leaf where the path runs out; find walks the same way and answers 1 or 0.

### Deleting

A node with **no child or one child** is easy: its child (or nothing) takes its place, spliced onto its parent.

A node with **two children** cannot simply be removed. Instead, find its **successor**, the smallest key in its right subtree (go right once, then left as far as possible). It fits between the two sides. Copy the successor's key into the node, then delete the successor from the right subtree; it has no left child, so that delete is the easy case.

In the default input, insert 50, 30, 70, 60, 80, 65 and then delete 50, which has two children. Its successor is 60 (right to 70, left to 60). 60 is copied into the root, and the old 60 node, whose only child is 65, is spliced out, so 65 becomes 70's left child.

### Height

Every operation costs one walk down, so its cost is the tree's **height** `h`. Keys inserted in random order give a height near `log n`. Sorted keys build a **skewed** tree, really a linked list, of height `n`. Balanced trees ([AVL](learn:trees/avl-rotations), [red-black](learn:trees/red-black-insert)) reshape themselves to avoid this.

## Steps

1. insert `k`: walk down from the root (left if `k` is smaller, else right) and attach `k` as a new leaf.
2. find `k`: walk down the same way until you meet `k` (output 1) or run out of nodes (output 0).
3. delete `k`: walk down to `k`. If it has at most one child, replace it with that child.
4. If it has two children, copy its successor's key into it and delete the successor from the right subtree.
5. At the end, the output holds the find answers, a 0, then the keys in order.

## Cost

| Operation | Cost |
|---|---|
| insert, find, delete | `O(h)` for height `h` |
| Balanced tree | `O(log n)` |
| Skewed tree | `O(n)` |
| Extra space | `O(n)` for the nodes |

## Pitfalls

- **Forgetting to relink.** A delete must update the parent's link (or the root) to the replacement; the sample does it by returning the new subtree root.
- **Inconsistent equal keys.** Decide where equal keys go (here: right) and use the same rule in insert, find and delete.

## When to use it

Use a BST when you need a set or map that also stays ordered: the smallest or largest key, the next key after a value, or all keys in a range.

## Related topics

- [Binary search](learn:searching/binary-search): the same halving, on a sorted array.
- [Pre-, in- and post-order](learn:trees/tree-traversals): in-order reads a BST sorted.
- [Binary heap and priority queue](learn:trees/binary-heap): a different tree order, for the smallest key only.
- [AVL tree: rotations](learn:trees/avl-rotations): keeping the height near `log n`.
