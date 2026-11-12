## Approach: a trie with pass counts

A beginning `p` of a name is a valid label exactly when the only name starting with `p` is the name itself. So we need, for many prefixes, the number of names that start with them. A **trie** answers that: insert every name letter by letter and add 1 to a counter `passes` in every node on the way. A node's `passes` is the number of names that start with its prefix.

Then, for each name, walk down from the root along its letters. Counts only shrink as the prefix grows, and the first node with `passes == 1` gives the shortest label. If every node on the path has `passes ≥ 2`, the whole name is the beginning of some other name, and the label is the whole name.

```
for name in names: insert(name)          # passes += 1 along the path
for name in names:
    node = root
    for k, c in enumerate(name):
        node = child(node, c)
        if node.passes == 1: label = name[: k + 1]; break
    else: label = name
```

### Walkthrough

For "dog", "door", "cat", "cattle": the nodes "d" and "do" have 2 passes and "dog" has 1, so "dog" needs all three letters; "doo" has 1 pass. On "cat" every node has 2 passes (shared with "cattle"), so the label is "cat"; for "cattle", "catt" has 1 pass.

**Complexity:** O(total letters) time and O(total letters) nodes. Comparing every name with every other name costs O(n² · L).
