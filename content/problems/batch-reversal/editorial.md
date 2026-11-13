## Approach: reverse batch by batch, stitching the ends

Put a dummy node before the line and let `before` point at it.

1. **Check the batch:** walk k nodes ahead of `before`. If the list ends first, stop: the rest stays as it is.
2. **Reverse it:** let `first = before.next`. Reverse the k nodes starting at `first` with the usual three pointers, leaving `rest` = the node after the batch.
3. **Stitch:** `before.next` = the batch's new first node (its old last), and `first.next = rest` (the old first is now last).
4. **Advance:** `before = first`, and repeat.

The hard part is keeping both ends linked correctly; the dummy node removes the special case of the very first batch.

### Walkthrough

`[1..8]`, `k = 3`: the batch 1, 2, 3 becomes 3, 2, 1 and 1 links to 4; the batch 4, 5, 6 becomes 6, 5, 4 and 4 links to 7; then only 7, 8 remain, fewer than 3, so they stay.

**Complexity:** O(n) time (each node is visited twice), O(1) extra space.
