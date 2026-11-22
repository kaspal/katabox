Simple chains like `[0, 1], [1, 2]` work. Try a person who makes two friendships as the first name of the pair, for example `[0, 1]` and then `[0, 2]`, and check who is still in a club with 1.
---
`find` returns the right root for the tree as it stands. The problem is in how two clubs are joined: which element gets its `parent` changed?
---
`parent[a] = b` overwrites `a`'s own link, so whatever `a` was attached to (its old club) is cut off. Join the roots instead: `ra, rb = find(a), find(b)`, and if they differ, `parent[ra] = rb`. Halving the path inside `find` (`parent[x] = parent[parent[x]]`) also keeps the trees shallow.
