## Approach: simulate with a while loop

Keep a step counter at 0. While the number is not 1, apply the rule (halve an even number; turn an odd one into `3n + 1`) and add one to the counter. When the loop stops, the counter is the answer; for `n = 1` the loop never runs.

The intermediate numbers can grow far beyond `n` (for n ≤ 10⁶ they stay below 10¹¹), so use 64-bit integers where the language distinguishes them.

**Complexity:** proportional to the number of steps (at most a few hundred for this range).
