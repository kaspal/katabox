## The idea

**Level order** visits a tree row by row: the root, then every node one step below it, then every node two steps below, each row from left to right. It is **breadth-first search** (BFS) on a tree.

The tool is a **queue**, first in, first out. Start with the root in the queue. Then, over and over, take the node at the front, write its value, and put its children (left, then right) at the back. Children always join the queue after everything already waiting, and that is exactly what keeps the rows in order: a node's children are written only after the whole row above them.

### The default tree

The values 50, 30, 70, 20, 40, 60, 80, 35 build a binary search tree (smaller values go left, equal or bigger go right). 50 is the root; 30 and 70 form the second row; 20, 40, 60 and 80 the third; and 35, the left child of 40, sits alone on the fourth. Level order writes 50, 30, 70, 20, 40, 60, 80, 35.

Watch the queue while it runs: after 50 it holds 30, 70; after 30, it holds 70, 20, 40; after 70 it holds the whole third row, four nodes. The queue never holds much more than one row, so the memory the walk needs is set by the tree's **width**, not its height.

## Steps

1. Build the BST from the values, then put the root in an empty queue.
2. While the queue is not empty, take the node at its front and write its value.
3. If the node has a left child, add it to the back of the queue.
4. If the node has a right child, add it to the back of the queue.
5. When the queue is empty, every node has been written, row by row.

## Cost

| Operation | Cost |
|---|---|
| Whole walk | `O(n)`: each node enters and leaves the queue once |
| Extra space | `O(w)` for the widest level, up to about `n/2` in a full tree |

## Pitfalls

- **Using a list's front as the queue.** In Python, `list.pop(0)` shifts every value and costs `O(n)`; use `collections.deque`. In JavaScript, `shift()` has the same problem on large arrays, so keep a head index instead.
- **Losing the row breaks.** If you need each row separately (for example, its sum), note the queue's length at the start of a row and take exactly that many nodes.

## When to use it

Use level order when the answer depends on depth: printing a tree row by row, the leftmost or rightmost node of each row, the widest row, or the nearest node with some property (BFS reaches the closest ones first).

## Related topics

- [Queue and circular buffer](learn:linear/queue-ring-buffer): the queue that drives the walk.
- [Pre-, in- and post-order](learn:trees/tree-traversals): the depth-first walks of the same tree.
- [Tree diameter](learn:trees/tree-diameter): BFS on a tree to find its longest path.
- Breadth-first search on graphs (coming soon): the same idea when nodes can be reached in more than one way.
