## Approach: shrink from the lower side

Put `i` at the first board and `j` at the last. Record `(j − i) · min(boards[i], boards[j])`. Then move the pointer at the **lower** board one step inwards, and repeat until `i = j`.

**Why dropping the lower board is safe:** say `boards[i] ≤ boards[j]`. Every other tank that uses board `i` pairs it with some board `k` between `i` and `j`; it is narrower than `j − i`, and its height is at most `boards[i]`. So none of them beats the tank just recorded, and board `i` can be discarded. Each step discards one board, so after n − 1 steps every board has been considered with its best partner.

### Walkthrough

`[2, 7, 4, 1, 8, 3, 6]`: (0, 6) gives 6 · 2 = 12, drop board 0; (1, 6) gives 5 · 6 = 30, drop board 6; (1, 5) gives 4 · 3 = 12, drop 5; (1, 4) gives 3 · 7 = 21, drop 1; the rest are smaller. The answer is 30.

**Complexity:** O(n) time, O(1) space. Checking every pair is O(n²).
