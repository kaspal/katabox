## Approach: sort by a two-part key

Pair each name with its time and sort the pairs by the key `(time, name)`. Tuples compare part by part: the times decide, and the names are compared only when the times are equal. String comparison is alphabetical, and a string that is the beginning of a longer one is smaller, as the board requires.

```
order = sort indices i by (times[i], names[i])
return [names[i] for i in order]
```

Sorting the indices (not the names alone) keeps every name together with its own time.

### Walkthrough

The pairs are (1810, ana), (1795, leo), (1810, bea), (2000, zoe). Sorted: (1795, leo), (1810, ana), (1810, bea), (2000, zoe), so the board reads leo, ana, bea, zoe.

**Complexity:** O(n log n) comparisons (each comparing at most 10 letters), O(n) extra space.
