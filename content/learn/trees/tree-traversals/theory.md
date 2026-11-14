## The idea

A **binary tree** is made of **nodes**; each node holds a value and links to at most two children, a left one and a right one. The top node is the **root**. Here the values build a **binary search tree** (BST): each new value starts at the root and goes left when it is smaller than the node, right when it is equal or bigger, until it finds a free spot. The default values 50, 30, 70, 20, 40, 60, 80 make a full tree three levels deep, with 50 at the root.

To **traverse** a tree is to visit every node once. The natural way is recursive: visit a node, its left subtree and its right subtree, in some order. The only choice is *when* to write the node's own value:

- **Pre-order**: the node first, then left, then right. 50, 30, 20, 40, 70, 60, 80.
- **In-order**: left, then the node, then right. 20, 30, 40, 50, 60, 70, 80.
- **Post-order**: left, then right, then the node. 20, 40, 30, 60, 80, 70, 50.

### Why in-order comes out sorted

In a BST, everything in a node's left subtree is smaller than it and everything in its right subtree is not. In-order writes the whole left subtree, then the node, then the whole right subtree, so the values come out in ascending order.

Pre-order writes a parent before its children, which is the order you need to copy a tree or print it as an outline. Post-order writes the children first, which is what you need when a node's answer depends on its children: sizes, heights, or deleting a tree from the leaves up.

## Steps

1. Insert the values one by one into a BST, starting from an empty tree.
2. Pre-order(node): if the node exists, write its value, then pre-order the left child, then the right child.
3. In-order(node): in-order the left child, write the value, in-order the right child.
4. Post-order(node): post-order the left child, then the right child, then write the value.
5. The output holds the three orders one after the other.

## Cost

| Operation | Cost |
|---|---|
| Each traversal | `O(n)`: every node is written once |
| Extra space | `O(h)` call stack for a tree of height `h` |

## Pitfalls

- **Forgetting the empty case.** Every recursive traversal must stop at a missing child, or it reads a value from nothing.
- **Deep recursion.** A tree built from sorted values is one long path, so the recursion is `n` calls deep and can overflow the stack. Use an explicit stack for very deep trees.

## When to use it

Use pre-order to copy or serialise a tree, in-order to read a BST in sorted order, and post-order whenever a node needs its children's results first, such as computing subtree sizes or freeing memory.

## Related topics

- [Recursion and the call stack](learn:foundations/recursion-call-stack): the stack every traversal uses.
- [Level order](learn:trees/level-order): visiting the same tree row by row with a queue.
- [BST: search, insert, delete](learn:trees/bst-operations): how the tree is built and changed.
- [Stack](learn:linear/stack): how to traverse without recursion.
