The count starts at `n` and drops by one exactly when a merge joins two **different** clubs. So the whole question is: are `a` and `b` already in the same club?
---
Give every club a representative. Relabelling every member of a club on each merge is correct, but a big club gets relabelled again and again.
---
Union-find: each player points to a parent, and following parents leads to the club's representative. Merge by pointing one representative at the other; shorten the paths as you follow them (path compression) and attach the smaller club under the larger (union by size).
