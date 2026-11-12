Always taking the largest token that fits can fail (see Example 1). Trying every combination is correct but explodes.
---
The last token paid has some value `c`. Then the rest is the best way to pay `amount − c`. The same smaller amounts come up again and again.
---
Fill `best[x]` for `x = 0 … amount`: `best[0] = 0`, and `best[x] = 1 + min(best[x − c])` over the tokens `c ≤ x` whose `best[x − c]` is reachable. The answer is `best[amount]`, or `-1` if it was never reached.
