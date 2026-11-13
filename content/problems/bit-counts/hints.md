Counting the bits of each number separately works, but the table can build on itself. How is the binary form of `i` related to that of `i / 2`?
---
Shifting right by one (`i >> 1`, integer division by 2) drops the last binary digit and keeps all the others.
---
So `bits[i] = bits[i >> 1] + (i & 1)`: the bits of `i` without its last digit, plus that last digit. Fill the table from 0 upwards.
