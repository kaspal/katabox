## Approach: one pass with a hash map

Scan the array once. For each `nums[i]`, compute `need = target - nums[i]`. If `need` is already in the map, return `[map[need], i]`; otherwise store `nums[i] -> i`.

Checking before inserting is what prevents using the same element twice.

### Walkthrough

`nums = [7, -3, 10, 5]`, `target = 2`:

| i | nums[i] | need | map before | result |
| --- | --- | --- | --- | --- |
| 0 | 7 | −5 | {} | store 7 → 0 |
| 1 | −3 | 5 | {7: 0} | store −3 → 1 |
| 2 | 10 | −8 | {7: 0, −3: 1} | store 10 → 2 |
| 3 | 5 | −3 | {7: 0, −3: 1, 10: 2} | −3 is there: return [1, 3] |

With `nums = [9, 2, 9]` and `target = 18`, the partner of the first 9 is 9 itself, but the map is still empty at `i = 0`, so an element is never paired with itself; the second 9 finds the first one.

**Complexity:** O(n) time, O(n) extra space. Trying every pair is O(n²), which is too slow for 10⁴ numbers.
