A copy editor suspects a long manuscript `text` contains passages pasted twice. Find the length of the longest piece of `text` (a contiguous run of letters) that appears **at least twice**, at different starting positions. The two appearances may overlap.

Return that length, or `0` if no letter sequence appears twice.

### Example 1

```
text = "mississippi"
Output: 4
```

"issi" starts at positions 1 and 4.

### Example 2

```
text = "abcd"
Output: 0
```

### Example 3

```
text = "aaaa"
Output: 3
```

"aaa" starts at positions 0 and 1.

### Constraints

- 1 ≤ text.length ≤ 10⁵
- `text` has lowercase English letters only.
