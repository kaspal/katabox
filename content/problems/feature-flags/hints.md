A user has every feature of question `q` exactly when `users[i] & q == q`: the user's number is a **superset** of the question's bits. Checking every user for every question is up to 10¹⁰ checks.
---
There are only 2¹⁶ = 65 536 possible numbers. Count how many users have each exact number, then compute for every possible question the number of users whose settings are a superset of it, once.
---
Superset sums: start with `count[m]` = users with exactly `m`. For each bit `b` and every `m` without bit `b`, add `count[m | b]` to `count[m]`. After all 16 bits, `count[m]` sums over all supersets of `m`. Each question is then a lookup.
