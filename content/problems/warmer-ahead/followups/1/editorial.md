## Idea

Wrapping around is the same as reading the season twice in a row. Keep the base problem's stack of days still waiting for a warmer noon, coldest on top, and walk `k` from 0 to `2n − 1` over day `i = k mod n`. While the day `d` on top is colder than day `i`, pop it and set its wait to `(i − d) mod n`. Push days only on the first pass, so each day enters the stack once; the second pass only answers days that are still waiting. A day that has a warmer day anywhere meets it within the next `n − 1` steps, so the days left on the stack are exactly those at the season's maximum, and they keep `0`.

## Complexity

O(n) time: 2n steps, and each day is pushed and popped at most once. O(n) extra memory for the stack.
