Adding up each range day by day can cost up to n steps per query. With many long queries, that is too slow.
---
If you knew the total rain from day 0 up to any day, could you get the total of a range with one subtraction?
---
Build `prefix` with `prefix[0] = 0` and `prefix[d + 1] = prefix[d] + rain[d]`. The answer to `[first, last]` is `prefix[last + 1] - prefix[first]`.
