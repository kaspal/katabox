## Approach: count, then cut once

Pressing the button `n` times (n songs) puts every song back in place, so only `r = k mod n` presses matter. After them, the last `r` songs come first and the others follow, in their original order. That is a single cut in the list:

1. Walk to the tail, counting `n`. An empty list is returned as is.
2. Compute `r = k mod n`. If `r = 0`, nothing changes.
3. Walk `n − r − 1` steps from the head to the **new tail**. The node after it is the **new head**.
4. Set `newTail.next = null` and `oldTail.next = oldHead`.

No node is created or copied: only two links change.

### Walkthrough

`playlist = [4, 8, 15, 16, 23]`, `k = 2`: n = 5, r = 2. The new tail is 3 − 1 = 2 steps after the head, the node 15; the new head is 16. Cutting after 15 and linking 23 → 4 gives `[16, 23, 4, 8, 15]`.

**Complexity:** O(n) time, O(1) extra space. Pressing the button `r` times one by one costs a walk to the end each time, O(n·r), which is O(n²) when r is about n/2: far too slow for 10⁵ songs.
