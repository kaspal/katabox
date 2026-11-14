## The idea

A linked list may loop: some node's `next` points back at an earlier node, and walking it never ends. Floyd's method finds out with two pointers and no extra memory. **slow** (the tortoise) moves one node per step, **fast** (the hare) moves two.

- If there is no cycle, fast reaches the end of the list and you are done.
- If there is a cycle, both pointers end up going round it. Once inside, fast gains one node on slow per step, so it must land exactly on slow; it cannot jump over it. They **meet**.

A second phase finds **where it starts**: send slow back to the head, leave fast at the meeting point, and move both **one** node per step. The node where they meet again is the start of the cycle.

### Why the second phase works

Say the tail before the cycle is `t` nodes long and the cycle is `c` nodes long. When the pointers first meet, fast has walked twice as far as slow, and the extra distance is a whole number of laps. So slow's own distance from the head is a whole number of laps, too.

Now walk `t` more steps from the meeting point. Together with slow's earlier distance, that adds up to `t` plus whole laps: exactly where `t` steps from the head would land, which is the cycle's start. And a pointer starting at the head reaches the start after the same `t` steps. Both arrive at once.

## Steps

1. Set `slow = fast = head`.
2. While `fast` and `fast.next` exist: move `slow` one node and `fast` two. If they are the same node, stop.
3. If `fast` reached the end, there is no cycle: return nothing.
4. Set `slow = head`; leave `fast` where they met.
5. While `slow` is not `fast`, move each one node.
6. Return `slow`: the first node of the cycle.

## Cost

| Measure | Cost |
|---|---|
| Detect the cycle | `O(n)`: slow meets fast before finishing one lap |
| Find its start | `O(n)`: `t` more steps |
| Extra space | `O(1)`: two pointers |

## Pitfalls

- **Checking only `fast`.** fast jumps two nodes, so test both `fast` and `fast.next` before moving it.
- **Comparing values instead of nodes.** Two different nodes can hold the same value. Compare the nodes themselves (`is` in Python, `===` in JavaScript).
- **Resetting the wrong pointer.** Only slow goes back to the head; fast must stay at the meeting point for the second phase to work.

## When to use it

Use it to check a linked list for a loop without extra memory, and more generally on any sequence where each value determines the next one: pseudo-random generators, or an array whose values are indices into itself, where it finds a duplicate in `O(1)` space. The same slow and fast pair also finds the middle of a list in one pass.

## Related topics

- [Singly linked list](learn:linear/singly-linked-list): the structure that can loop.
- [Two pointers](learn:searching/two-pointers): two indices moving under simple rules.
- Hash tables (coming soon): detect a cycle by remembering visited nodes, at `O(n)` space.
