## The idea

A **priority queue** hands out values smallest first, however they arrived. A **binary heap** does this cheaply. It is a binary tree stored in a plain array: the root is at position 0, and the children of position `i` are at `2i + 1` and `2i + 2` (so its parent is `(i − 1) // 2`). The tree fills row by row, so no links are stored.

This one is a **min-heap**: every parent is at most its children, so the smallest value is always at the root.

- **push**: put the value at the end of the array, then **sift it up**: while it is smaller than its parent, swap the two.
- **pop**: take the root. Move the last value to the root, then **sift it down**: while it is bigger than its smaller child, swap it with that child. On an empty heap, pop returns `−1`.

### The default input

push 42, 17, 8, 23, 4 leaves the array as 4, 8, 17, 42, 23. The last push shows sifting up: 4 starts at position 4, beats its parent 23, moves to position 1, beats 8, and becomes the root.

The first pop takes 4 and moves 23 to the root. Its children are 8 and 17; the smaller is 8, so 23 swaps with 8 and stops above 42. Five pops output 4, 8, 17, 23, 42: the values in sorted order.

## Steps

1. push `x`: append `x`; let `i` be its position.
2. While `i > 0` and `x` is smaller than its parent, swap them and move `i` to the parent.
3. pop: if the heap is empty, output `−1`. Otherwise output the root and remove the last value.
4. If values remain, put that last value at the root.
5. While it has a child and is bigger than its smaller child, swap it with that child.

## Cost

| Operation | Cost |
|---|---|
| push | `O(log n)`: one path up |
| pop | `O(log n)`: one path down |
| peek at the smallest | `O(1)`: it is at position 0 |
| Extra space | `O(n)`, the array itself, with no links |

## Pitfalls

- **Comparing with the wrong child.** Sifting down must swap with the *smaller* child; swapping with the other would put a bigger value above a smaller one.
- **Children past the end.** A node can have two children, one or none. Check `2i + 1 < size` before reading a child.
- **Expecting a sorted array.** Only the root is guaranteed; the array 4, 8, 17, 42, 23 is a valid heap but not sorted.

## When to use it

Use a heap whenever you repeatedly need the smallest (or largest) of a changing set: scheduling tasks by priority, merging sorted lists, keeping the top k scores, and Dijkstra's shortest paths. Python has `heapq` (a min-heap); in JavaScript you write one like the sample.

## Related topics

- [Heap Sort](learn:sorting/heap-sort): a max-heap built in place, then emptied into a sorted array.
- [Queue and circular buffer](learn:linear/queue-ring-buffer): the plain first-in, first-out queue.
- [BST: search, insert, delete](learn:trees/bst-operations): a fully ordered tree, at a higher cost.
- Dijkstra's shortest paths (coming soon): a priority queue at work.
