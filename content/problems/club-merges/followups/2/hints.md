An undo always takes back the most recent merge still in effect. Which data structure hands you "the most recent" item?
---
Path compression rewires many players at once, which is hard to take back. Union by size alone still keeps every tree O(log n) tall, and then each merge changes just one parent link and one size.
---
On each merge, push the root that was attached, or a marker for "changed nothing". On `[0]`, pop: if it is a real root, set its parent back to itself, subtract its size from the root it had joined, and add one club back.
