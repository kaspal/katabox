Checking every pair is O(n²): with 10⁵ numbers that is about 5·10⁹ pairs. Can you count
the partners of each number without looking at them one by one?
---
When you reach `nums[j]`, every earlier position holding `target − nums[j]` makes a pair
with it.
---
Keep a map from value to how many times you have seen it. For each number, add the count
of its partner, then count the number itself.
