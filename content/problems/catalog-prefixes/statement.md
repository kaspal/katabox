A small library is testing a new search box. While a reader types, the box shows how many book titles in the catalogue **begin with** the letters typed so far.

`titles` is the catalogue (a title may appear more than once: each copy counts) and `typed` lists what readers typed. For each entry of `typed`, return how many titles start with it. A title starts with itself.

### Example 1

```
titles = ["moon", "moonlight", "mars", "moon", "map"], typed = ["mo", "m", "moonl", "x"]
Output: [3, 5, 1, 0]
```

"mo" matches both copies of "moon" and "moonlight"; every title begins with "m"; only "moonlight" begins with "moonl".

### Example 2

```
titles = ["river"], typed = ["river", "rivers", "r"]
Output: [1, 0, 1]
```

"rivers" is longer than the only title, so nothing begins with it.

### Example 3

```
titles = ["ab", "abc", "b"], typed = ["abc", "ab", "a"]
Output: [1, 2, 2]
```

### Constraints

- 1 ≤ titles.length ≤ 10⁴
- 1 ≤ typed.length ≤ 2·10⁴
- 1 ≤ titles[i].length, typed[j].length ≤ 20
- Titles and typed text use lowercase English letters only
