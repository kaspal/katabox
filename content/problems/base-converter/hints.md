Go through an ordinary integer: first read the number, then write it again.
---
Reading, left to right: `value = value · source + digit`. Writing: the last digit in base `target` is `value mod target`; then divide by `target` and repeat.
---
Writing produces the digits from right to left, so reverse them at the end, and handle 0 separately (the loop would write nothing).
