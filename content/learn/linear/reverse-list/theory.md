## The idea

To reverse a singly linked list in place, you do not move any values. You turn every arrow around: each node's `next` should point at the node that used to come **before** it. One walk from head to end does it, with three pointers:

- `cur`, the node being flipped;
- `prev`, the node before it, which is where `cur.next` must point now (nothing, for the first node, which becomes the new end);
- `nxt`, the rest of the list, saved before the flip destroys the only link to it.

Each round saves `cur.next` in `nxt`, points `cur.next` back at `prev`, and moves forward: `prev` becomes `cur`, and `cur` becomes `nxt`. When `cur` runs off the end, `prev` is the old last node, which is the new head.

### Why the saved pointer matters

The moment you write `cur.next = prev`, the node no longer knows where the rest of the list is. If you had not saved `nxt` first, everything after `cur` would be unreachable. The order "save, flip, advance" is the whole algorithm.

## Steps

1. Set `prev = None` and `cur = head`.
2. While `cur` exists, save `nxt = cur.next`.
3. Flip: `cur.next = prev`.
4. Advance: `prev = cur`, then `cur = nxt`.
5. When `cur` is empty, return `prev` as the new head.

## Cost

| Measure | Cost |
|---|---|
| Time | `O(n)`: one visit per node, one flip each |
| Extra space | `O(1)`: three pointers |
| Recursive version | `O(n)` time, but `O(n)` call stack |

## Pitfalls

- **Flipping before saving.** `cur.next = prev` first, then `nxt = cur.next`, gives `nxt = prev`: on the first round that is nothing, so the loop stops after one node and the rest of the list is lost.
- **Returning the old head.** After the loop, the old head is the **last** node. The new head is `prev`, not `head` and not `cur` (which is empty).
- **Forgetting the old head's link.** Starting with `prev = None` is what makes the old head point at nothing. Start at the second node instead and the old head keeps its `next`, so the last two nodes point at each other: a cycle.
- **Recursion on long lists.** A recursive reverse is elegant but uses one stack frame per node; a list of a million nodes can overflow the stack.

## When to use it

Reversal is a building block more than a goal. It appears in checking whether a list is a palindrome (reverse the second half and compare), in reversing groups of `k` nodes, in adding numbers stored digit by digit, and in reordering a list by interleaving its two halves. Any task that needs to read a singly linked list backwards can reverse it, read it, and reverse it again.

## Related topics

- [Singly linked list](learn:linear/singly-linked-list): the structure being rewired.
- [Recursion and the call stack](learn:foundations/recursion-call-stack): the recursive version and its stack.
- [Floyd cycle detection](learn:linear/floyd-cycle): another pointer walk; with a fast pointer it also finds the middle.
- [Two pointers](learn:searching/two-pointers): reading from both ends, which arrays allow directly.
