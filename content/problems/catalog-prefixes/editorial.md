## Approach: a trie with pass counts

A **trie** stores strings letter by letter: the root is the empty prefix, and the child of a node along letter `c` is that prefix followed by `c`. Titles with a common beginning share the nodes of that beginning.

Give every node a counter `passes`. Inserting a title walks from the root along its letters, creating missing children, and adds 1 to `passes` of every node it reaches. A node's `passes` is then the number of titles whose path goes through it, which is the number of titles that **begin with** its prefix.

```
insert(title):
    node = root
    for c in title:
        node = child(node, c)   # created if missing
        node.passes += 1
count(text):
    node = root
    for c in text:
        if no child(node, c): return 0
        node = child(node, c)
    return node.passes
```

Repeated titles simply pass through the same nodes again, so each copy counts.

### Walkthrough

With "moon", "moonlight", "mars", "moon", "map", the node for "m" is passed 5 times, "mo" 3 times (two "moon" and "moonlight"), "moonl" once. The search "x" stops at the root: no child "x", so 0.

**Complexity:** O(total letters of the titles) to build and O(length) per search, so O(T + Q) overall, with O(T) nodes. Checking every title against every typed text costs O(N · Q · L).
