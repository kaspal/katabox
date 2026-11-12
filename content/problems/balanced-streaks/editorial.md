## Approach: equal prefix scores

Define the running score `s[0] = 0` and `s[j] = s[j − 1] + (+1 for W, −1 for L, 0 for D)`. A run covering matches `i + 1 … j` has wins − losses = `s[j] − s[i]`, so it is balanced exactly when `s[i] == s[j]`. The answer is the number of pairs `i < j` of positions with equal scores.

```
seen = {0: 1}; score = 0; count = 0
for c in results:
    score += 1 if c == "W" else -1 if c == "L" else 0
    count += seen.get(score, 0)    # every earlier equal score closes a balanced run
    seen[score] = seen.get(score, 0) + 1
```

### Walkthrough

"WWLL": scores 0, 1, 2, 1, 0. The pairs of equal scores are (1, 1), which is "WL" in the middle, and (0, 0), which is the whole string. Count 2.

**Complexity:** O(n) time and O(n) memory for the map. The answer can reach about 5·10⁹ (a season of draws), so it needs 64-bit integers. Checking every run is O(n²).
