Checking every pair works, but it is O(n²). What do you need to know about the numbers you have already seen?
---
For each number x, the partner you need is target − x. Can you look that up in O(1)?
---
Keep a hash map from value to index while you scan. Check for the partner before inserting the current number.
