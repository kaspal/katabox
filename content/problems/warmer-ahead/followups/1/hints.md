Imagine the season written out twice in a row. A day's answer on the circle is its answer in the first copy of that doubled list.
---
The base problem's stack of days still waiting works on the doubled list too, and you do not need to build it: loop `k` from 0 to `2n − 1` and look at day `k mod n`.
---
Push days only during the first pass (`k < n`); in the second pass, only pop. When a warmer day `i` pops day `d`, the wait is `(i − d) mod n`. Days still on the stack at the end are the warmest of the season and keep 0.
