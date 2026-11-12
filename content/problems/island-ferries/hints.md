Start with `n` groups of one island. What does each ferry line do to the number of groups?
---
A line between two islands already in the same group changes nothing; a line between two different groups merges them, so the count drops by one. You need a fast way to ask "which group is this island in?".
---
Use a disjoint-set (union-find): `parent[i]` points towards the group's representative. `find` follows the pointers (shortening the path as it goes); `union` links the two representatives. Count the successful unions and return `n` minus that count.
