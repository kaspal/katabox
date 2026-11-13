At the end of a youth games, the organisers print the medal table. `countries[i]` won `medals[i] = [gold, silver, bronze]`.

Countries are ranked by gold medals (more first); ties are broken by silver, then by bronze (more first). Countries with exactly the same medals are listed by name in alphabetical order.

Return the country names in table order.

### Example 1

```
countries = ["norway", "chile", "kenya", "japan"],
medals = [[2, 1, 0], [2, 1, 3], [0, 5, 5], [2, 3, 0]]
Output: ["japan", "chile", "norway", "kenya"]
```

Three countries have 2 golds; japan has the most silvers; chile beats norway on bronze. Kenya's 10 medals do not include a gold.

### Example 2

```
countries = ["peru", "fiji"], medals = [[1, 0, 0], [1, 0, 0]]
Output: ["fiji", "peru"]
```

### Example 3

```
countries = ["malta"], medals = [[0, 0, 0]]
Output: ["malta"]
```

### Constraints

- 1 ≤ countries.length = medals.length ≤ 10⁴
- Names have 1 to 20 lowercase English letters and are all different.
- 0 ≤ gold, silver, bronze ≤ 1000
