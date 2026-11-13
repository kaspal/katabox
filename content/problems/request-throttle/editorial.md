## Approach: a sliding log per user

Keep a map from user to a queue (deque) of the times of that user's **allowed** requests, oldest first.

For an operation of user `u` at time `t`:

1. drop times `≤ t − window` from the front of `u`'s queue; they are out of the window `(t − window, t]` and, since times never decrease, they never come back;
2. `"count"`: report the queue's length;
3. `"request"`: if the length is below `limit`, report 1 and push `t`; otherwise report 0 (a refused request is not stored).

Each allowed request is pushed once and popped at most once, so the work is linear overall. Each queue holds at most `limit` times.

### Walkthrough

Example 1: user 1's queue becomes [1], [1, 3]; at time 5 it is full, so 0; at time 11, the 1 drops out (1 ≤ 11 − 10), leaving [3], so the request is allowed: [3, 11]. User 2 starts empty.

**Complexity:** O(1) amortised per operation, O(users · limit) space.
