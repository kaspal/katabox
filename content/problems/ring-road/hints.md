A road lies on the loop exactly when removing it keeps its two villages connected. Testing that road by road works, but it is a search per road: O(n²).
---
Look at a village with only one road (a dead end). It cannot be on the loop: a loop enters and leaves each of its villages by two different roads.
---
Remove dead ends repeatedly: each removal lowers the road count of its neighbour, which may become a new dead end (use a queue). What survives is exactly the loop; count it.
