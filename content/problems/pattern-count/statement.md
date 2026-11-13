A weaver checks a long strip of patterned cloth, written as the string `text`, for a motif, the string `pattern`. Occurrences may overlap.

Return how many positions `i` there are where `pattern` appears in `text` starting at `i`.

### Example 1

```
text = "tatatat", pattern = "tat"
Output: 3
```

At positions 0, 2 and 4.

### Example 2

```
text = "hello", pattern = "xyz"
Output: 0
```

### Example 3

```
text = "aaaa", pattern = "aa"
Output: 3
```

### Constraints

- 1 ≤ text.length ≤ 2·10⁵
- 1 ≤ pattern.length ≤ 10⁵
- Both strings have lowercase English letters only.
