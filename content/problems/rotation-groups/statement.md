A jeweller engraves bracelets with lowercase letters all the way round. A bracelet has no start, so `"lamp"`, `"ampl"`, `"mpla"` and `"plam"` are all the **same** bracelet read from different starting points. Reading it backwards is not allowed: `"pmal"` is a different bracelet.

`codes` lists the engravings as they were written down. Return how many **different** bracelets there are.

### Example 1

```
codes = ["lamp", "mpla", "palm", "ampl", "lmpa"]
Output: 2
```

`"lamp"`, `"mpla"` and `"ampl"` are one bracelet. `"palm"` and `"lmpa"` are another: start reading `"palm"` at its third letter.

### Example 2

```
codes = ["aa", "aa", "a"]
Output: 2
```

Different lengths are always different bracelets.

### Example 3

```
codes = ["abc", "bca", "cab", "cba"]
Output: 2
```

### Constraints

- 1 ≤ codes.length ≤ 2·10⁴
- 1 ≤ codes[i].length ≤ 12
- Codes contain only lowercase letters `a`–`z`
