A beginning `p` of a name is a good label exactly when **no other** name starts with `p`, that is, when exactly one name (this one) starts with `p`. Comparing each name with all others is O(n²·L).
---
Put all names in a **trie** and count, in every node, how many names pass through it. That count is the number of names that start with the node's prefix.
---
For each name, walk down its letters; the first node whose count is 1 ends the label. If you reach the end of the name without finding one, the label is the whole name.
