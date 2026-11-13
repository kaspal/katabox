Comparing distances does not need square roots: compare x² + y² instead, exactly, in integers.
---
Rank each stall by the pair (squared distance, index). The answer is the k smallest pairs in order.
---
Either sort all n pairs (O(n log n)), or keep a max-heap of the best k seen so far (O(n log k)), then sort those k. Picking the minimum k times with a full scan each time is O(n·k), too slow.
