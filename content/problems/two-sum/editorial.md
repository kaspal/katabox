## Approach: one pass with a hash map

Scan the array once. For each `nums[i]`, compute `need = target - nums[i]`. If `need` is already in the map, return `[map[need], i]`; otherwise store `nums[i] -> i`.

Checking before inserting is what prevents using the same element twice.

**Complexity:** O(n) time, O(n) extra space.
