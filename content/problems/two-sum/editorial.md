## Approach: one pass with a hash map

Scan the array once. For each `nums[i]`, compute `need = target - nums[i]`. If `need` is already in the map, return `[map[need], i]`; otherwise store `nums[i] -> i`.

Checking before inserting is what prevents using the same element twice.

### Walkthrough

`nums = [-6, 14, 3, 20, -1]`, `target = 14` (Example 2):

| i | nums[i] | need | map before | result |
| --- | --- | --- | --- | --- |
| 0 | −6 | 20 | {} | store −6 → 0 |
| 1 | 14 | 0 | {−6: 0} | store 14 → 1 |
| 2 | 3 | 11 | {−6: 0, 14: 1} | store 3 → 2 |
| 3 | 20 | −6 | {−6: 0, 14: 1, 3: 2} | −6 is there: return [0, 3] |

The line holding 14 needs a 0, which never appears; it is not paired with itself because it looks for its partner before it is stored. With `nums = [9, 2, 9]` and `target = 18`, the first 9 finds an empty map; the second 9 finds the first one: `[0, 2]`.

**Complexity:** O(n) time, O(n) extra space. Trying every pair is O(n²), which is too slow for 10⁵ numbers.
