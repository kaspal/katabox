## Approach: one pass with a hash map

Scan the array once. For each `nums[i]`, compute `need = target - nums[i]`. If `need` is already in the map, return `[map[need], i]`; otherwise store `nums[i] -> i`.

Checking before inserting is what prevents using the same element twice.

### Walkthrough

`nums = [3, 2, 4]`, `target = 6`:

| i | nums[i] | need | map before | result |
| --- | --- | --- | --- | --- |
| 0 | 3 | 3 | {} | store 3 → 0 |
| 1 | 2 | 4 | {3: 0} | store 2 → 1 |
| 2 | 4 | 2 | {3: 0, 2: 1} | 2 is there: return [1, 2] |

At `i = 0` the partner of 3 is 3 itself, but the map is still empty, so the element is not paired with itself.

**Complexity:** O(n) time, O(n) extra space. Trying every pair is O(n²), which is too slow for 10⁴ numbers.
