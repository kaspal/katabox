## Approach: superset sums over all 2¹⁶ masks

A user answers "yes" to question `q` when `user & q == q`. Instead of testing every pair, precompute the answer for every possible question.

1. `count[m]` = the number of users whose settings are exactly `m`.
2. For each bit `b` (0 to 15), for every mask `m` that does **not** have bit `b`: `count[m] += count[m | b]`.
3. Answer each question with `count[q]`.

**Why step 2 works.** After processing bits `0 … b`, `count[m]` holds the number of users whose settings equal `m` on the bits above `b` and contain `m` on bits `0 … b`. Adding `count[m | b]` brings in the users that have bit `b` while `m` does not ask for it. After the last bit, `count[m]` counts every superset of `m`. This is the "sum over supersets" dynamic programming.

Only the bits that actually occur need processing: with `B` = the highest bit used, the table has 2ᴮ entries.

### Walkthrough

`users = [5, 7, 2, 15]`: `count[5] = count[7] = count[2] = count[15] = 1`. After the sums, `count[5]` = users ⊇ 0101 = {5, 7, 15} = 3, `count[8]` = {15} = 1, `count[0]` = 4.

**Complexity:** O(2ᴮ · B + n + q) time and O(2ᴮ) memory, at most about 10⁶ steps. Checking every user for every question is O(n · q).
