## Approach: BFS over skill masks

Only the set of covered skills matters, so the states are the masks `0 .. 2^skills − 1`. From state `m`, adding volunteer `p` leads to `m | people[p]` and costs one team member.

Run a BFS from mask 0 (nobody yet, nothing covered). The first time the full mask `2^skills − 1` is reached, its distance is the smallest team size. If the BFS ends without reaching it, return −1.

Duplicate volunteers lead to the same moves, so de-duplicating them first saves time.

### Walkthrough

Example 1, full mask 1111: from 0, one volunteer reaches 0011, 0100, 1100 or 1001. From 0011, adding 1100 reaches 1111: distance 2.

**Complexity:** O(2^skills · n), about 10⁶ steps. Trying teams by size is O(C(n, k)) for each size k, which explodes when the answer is large (a team of 14 among 60 volunteers).
