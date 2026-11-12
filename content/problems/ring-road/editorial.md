## Approach: peel off the dead ends

Count the roads at every village (its **degree**). A village of degree 1 is a dead end and cannot be on the loop, since a loop arrives at each of its villages by one road and leaves by another. Remove it: its only neighbour loses a road. That may turn the neighbour into a new dead end, and so on.

```
queue = villages with degree 1
while queue:
    v = pop; removed[v] = true
    for each neighbour u of v not removed:
        degree[u] -= 1
        if degree[u] == 1: push u
return number of villages not removed
```

**Why what is left is the loop.** Villages on the loop always keep their two loop roads, so their degree never drops below 2 and they are never removed. Every other village hangs from the loop on a tree-shaped branch, and a finite tree always has a leaf, so peeling continues until every branch is gone. So exactly the loop survives.

### Walkthrough

Example 3: dead ends 5 and 6. Removing 5 makes 4 a dead end; removing 6 drops village 0 to degree 2; removing 4 drops village 2 to degree 2. Villages 0, 1, 2 and 3 remain: 4.

**Complexity:** O(n) time and memory: each village is removed at most once and each road looked at twice. Removing each road in turn and searching for another path is O(n²).
