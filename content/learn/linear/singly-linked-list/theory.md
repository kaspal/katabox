## The idea

A singly linked list stores each value in its own **node**, and each node holds one pointer: `next`, the node after it. The list itself is just `head`, a pointer to the first node; the last node's `next` is empty. Nodes can sit anywhere in memory, so adding or removing one never shifts the others. You only rewire pointers.

The price is that you can only move forward, one node at a time. There is no `a[i]`: reaching the fifth node means following four links.

- **Insert at the front** is instant: the new node points at the current head, then `head` points at the new node.
- **Insert at the back** has to walk the whole list to find the last node, since this version keeps no `tail` pointer. That walk is `O(n)`.
- **Delete by value** walks with two pointers: `cur` looks at each node, and `prev` trails one node behind. When `cur` holds the value, `prev.next` skips over it. If the match is the head itself, there is no `prev`, so `head` moves on instead.

## Steps

1. front `x`: make a node for `x`, set its `next` to `head`, then set `head` to it.
2. back `x`: if the list is empty, the node becomes `head`. Otherwise walk `cur` from `head` until `cur.next` is empty, then set `cur.next` to the node.
3. delete `x`: set `prev` to nothing and `cur` to `head`. While `cur` exists and its value is not `x`, move both one node on.
4. If `cur` ran off the end, `x` is not in the list: do nothing.
5. If `prev` is empty, set `head = cur.next`; otherwise set `prev.next = cur.next`.

## Cost

| Operation | Cost |
|---|---|
| Insert at the front | `O(1)` |
| Insert at the back (no tail pointer) | `O(n)`: walk to the last node |
| Delete or find by value | `O(n)` |
| Extra space | One `next` pointer per node |

## Pitfalls

- **Linking in the wrong order.** At the front, set `node.next = head` before `head = node`; the other way round, the old list is lost.
- **Deleting the head.** It has no `prev`, so it needs its own case. A "dummy" node placed before the head removes that special case.
- **Walking off the end.** Check that `cur` exists before reading `cur.value` or `cur.next`.

## When to use it

Linked lists shine when you already hold a pointer to the spot where things change: splicing lists together, removing a node you have in hand, or building a stack with `O(1)` front operations and no resizing. For reading by position or scanning lots of values, a dynamic array is faster in practice.

## Related topics

- [Doubly linked list](learn:linear/doubly-linked-list): adds a back pointer, so both ends are `O(1)`.
- [Reversing a list](learn:linear/reverse-list): rewires every `next` in one pass.
- [Floyd cycle detection](learn:linear/floyd-cycle): two pointers moving at different speeds.
- [Dynamic array: insert and delete](learn:linear/dynamic-array-ops): the array side of the same trade-off.
