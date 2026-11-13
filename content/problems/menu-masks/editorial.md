## Approach: every set as a bit mask

A number `mask` between 1 and 2ⁿ − 1 describes a set of dishes: bit `i` set means dish `i` is in. Enumerating all masks enumerates all non-empty sets.

For each mask, total the prices of the dishes whose bit is set (checking `mask >> i & 1` for each `i`). If the total equals the budget, record the list of those `i` in increasing order.

Mask order is not the required order: mask 0b1001 (dishes 0 and 3) comes after 0b0110 (dishes 1 and 2), but `[0, 3]` sorts before `[1, 2]`. So sort the recorded lists lexicographically at the end. (In PHP, `sort` compares arrays by length first, so a custom comparison is needed.)

**Complexity:** O(n · 2ⁿ) time, about 5·10⁵ steps for n = 15, plus sorting the answers.
