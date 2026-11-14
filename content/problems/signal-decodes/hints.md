Trying every split one by one is hopeless: a message of 1s and 2s has a number of readings that grows like the Fibonacci numbers. Count the readings without listing them.
---
Let `ways[i]` be the number of readings of the first `i` digits. The last letter of such a reading is either the digit `i` alone (allowed unless it is `0`) or the two digits `i-1, i` together (allowed when they form 10 to 26).
---
`ways[0] = 1`, and `ways[i] = (digit i ≠ 0 ? ways[i-1] : 0) + (digits i-1, i form 10..26 ? ways[i-2] : 0)`, all modulo 1 000 003. The answer is `ways[n]`; two variables are enough instead of a whole array. Remember to skip the `#`.
