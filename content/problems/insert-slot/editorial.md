## Approach: one pass in three phases

Walk through the sorted slots with an index `i`:

1. **Left:** while `slots[i].end < fresh.start`, the slot is entirely before the new one: copy it.
2. **Overlap:** while `slots[i].start ≤ fresh.end`, the slot shares a moment with the (growing) new slot: set `fresh = [min(starts), max(ends)]` and skip it.
3. Add `fresh`, then copy the remaining slots: they start after `fresh` ends.

Because the old slots were sorted and separate, the result is sorted and separate too.

### Walkthrough

Example 1: [1, 3] ends before 7, so it is copied. [6, 8] starts at 6 ≤ 10: fresh becomes [6, 10]. [10, 12] starts at 10 ≤ 10: fresh becomes [6, 12]. Nothing remains.

**Complexity:** O(n) time.
