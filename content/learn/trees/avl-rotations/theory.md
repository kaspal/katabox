## The idea

An **AVL tree** is a binary search tree that repairs itself after every insert, so its height stays near `log n` whatever order the keys arrive in. Each node stores its **height** (a leaf has 1, an empty subtree 0), and at every node the **balance**, left height minus right height, must stay −1, 0 or 1.

insert walks down as in a plain BST (equal keys go right) and attaches a new leaf. On the way back up, each node on the path recomputes its height and balance. Only those nodes can break the rule, and only by one, so a bad balance is always +2 or −2. The fix depends on where the new key went:

- **Left-left** (+2, the key is smaller than the left child's key): one **right rotation**. The left child moves up, the node becomes its right child, and the child's old right subtree becomes the node's left subtree.
- **Right-right** (−2, the key is not smaller than the right child's key): the mirror, one **left rotation**.
- **Left-right** and **right-left** (the key went to the inner side): first rotate the child the other way, which straightens the zigzag, then rotate the node.

A rotation keeps the keys in the same in-order sequence, so the tree is still a valid BST.

### The default input

10, 20, 30, 40, 50, 25. 30 leaves 10 at −2, right-right: a left rotation puts 20 on top with children 10 and 30. 50 then makes 30 right-right, and a left rotation puts 40 in its place with children 30 and 50. 25 goes right at 20, left at 40, left at 30. The root 20 ends at −2 with the key on the inner side: right-left. 40 rotates right, lifting 30, then 20 rotates left, so 30 becomes the root with 20 (children 10 and 25) on its left and 40 (right child 50) on its right. The pre-order output is 30, 20, 10, 25, 40, 50.

## Steps

1. Walk down as in a BST and attach the key as a new leaf of height 1.
2. Coming back up, set each node's height to 1 plus its taller child's, and compute its balance.
3. Balance +2: if the key is not smaller than the left child's key, rotate the left child left first; then rotate the node right.
4. Balance −2: if the key is smaller than the right child's key, rotate the right child right first; then rotate the node left.
5. At the end, output the keys in pre-order.

## Cost

| Operation | Cost |
|---|---|
| insert | `O(log n)`: one path down and back up |
| One rotation | `O(1)`: three links and two heights |
| Height | at most about `1.44 log n` |
| Extra space | `O(n)` for the nodes |

## Pitfalls

- **Updating heights in the wrong order.** After a rotation, update the node that moved down first: the one that moved up sits above it.
- **A single rotation on a zigzag.** On the inner side, one rotation only tips the imbalance the other way. Straighten first.
- **Losing the new top.** A rotation returns the subtree's new top; the parent's link (or the root) must take it.

## When to use it

Use an AVL tree when you need guaranteed `O(log n)` search and insert on an ordered set or map, even when keys arrive sorted. It stays a little flatter than a red-black tree, so lookups are slightly faster, at the price of more rotations.

## Related topics

- [BST: search, insert, delete](learn:trees/bst-operations): the plain tree an AVL tree keeps balanced.
- [Red-black tree: insert](learn:trees/red-black-insert): a looser balance kept with colours instead of heights.
- [B-tree: insert and split](learn:trees/b-tree-insert): balance by splitting nodes that hold several keys.
- [Pre-, in- and post-order](learn:trees/tree-traversals): the pre-order walk that writes the output.
