## Approach: an if / else-if chain per score

Go through the scores in order and append one letter for each. Testing the thresholds from the highest down means each test only needs a lower bound:

- `score >= 90` → `"A"`
- else `score >= 80` → `"B"`
- else `score >= 70` → `"C"`
- else `score >= 60` → `"D"`
- else `"F"`

When the test `score >= 80` runs, the score is already known to be below 90, so no upper bound is needed.

**Complexity:** O(n) time, O(n) for the result.
