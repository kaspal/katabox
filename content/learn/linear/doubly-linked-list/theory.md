## The idea

In a doubly linked list every node holds two pointers: `next`, the node after it, and `prev`, the node before it. The list keeps both ends, `head` and `tail`. With links in both directions, you can add or remove a node at either end without walking anywhere, so the list works as a **deque**: push and pop at the front and at the back, each in `O(1)`.

The catch is bookkeeping. Every change touches up to four pointers, and the ends need care: when the list has one node, that node is both `head` and `tail`, and removing it must clear both.

In a singly linked list, pop-back is the slow one: finding the node before the tail means walking from the head. Here `tail.prev` gives it directly.

In this topic, pops on an empty list return `−1`. At the end, the output reads the list forwards from `head` and then backwards from `tail`, which checks that both directions of links agree.

## Steps

1. front `x`: link the new node before `head` both ways; if the list was empty, it is also the `tail`. Set `head` to it.
2. back `x`: link the new node after `tail` both ways; if the list was empty, it is also the `head`. Set `tail` to it.
3. pop-front: if empty, return `−1`. Otherwise set `head = head.next`, clear the new head's `prev` (or `tail`, if the list is now empty), and return the old value.
4. pop-back: the same with `tail`, `prev` and `next` swapped.

## Cost

| Operation | Cost |
|---|---|
| Push at either end | `O(1)` |
| Pop at either end | `O(1)` |
| Remove a node you already hold | `O(1)` |
| Find by value or position | `O(n)` |
| Extra space | Two pointers per node |

## Pitfalls

- **Updating one direction only.** Setting `next` but not `prev` (or the other way round) makes the forward and backward readings disagree.
- **The one-node case.** Popping the last node must clear both `head` and `tail`; forgetting one leaves a pointer to a removed node.
- **Order of updates.** Save the node you are removing before moving `head` or `tail` off it, or its value is lost.

## When to use it

Use a doubly linked list when you need to remove or move nodes in the middle in `O(1)`, given a pointer to them: the recency list of an LRU cache, an editor's undo and redo, or a playlist you can step through both ways. Paired with a hash map that points at nodes, it gives `O(1)` lookups and `O(1)` reordering.

## Related topics

- [Singly linked list](learn:linear/singly-linked-list): one pointer per node, back insert `O(n)` without a tail.
- [LRU cache](learn:linear/lru-cache): a hash map plus a doubly linked list.
- [Deque and monotonic queue](learn:linear/deque-monotonic-queue): what a deque is good for.
- [Queue and circular buffer](learn:linear/queue-ring-buffer): an array-based alternative with no pointers.
