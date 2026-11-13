## Approach: build each entry from a smaller one

`i >> 1` is `i` without its last binary digit, and `i & 1` is that last digit. So

`bits[0] = 0` and `bits[i] = bits[i >> 1] + (i & 1)`.

Since `i >> 1 < i`, the entry needed is always already in the table.

### Walkthrough

bits[6]: 6 = 110, 6 >> 1 = 3 = 11 with 2 bits, and 6 & 1 = 0, so 2. bits[7] = bits[3] + 1 = 3.

**Complexity:** O(n) time, one step per entry. Counting each number's bits one by one is O(n log n), also fine here; the table version shows how a bit-level identity turns into a DP.
