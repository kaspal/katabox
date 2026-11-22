Ines publishes the results of her running club's Sunday race. Her script sorts runners by finishing time, but whenever two runners cross the line together, the club members complain that the order looks backwards.

Given arrays `names` and `times`, where runner `names[i]` finished in `times[i]` seconds, return the names ordered from fastest to slowest time. Runners with the same time are ordered by name in ascending alphabetical order (compare character by character; digits come before letters, and a name comes before any longer name that starts with it).

The editor starts with that code. Find the bug and fix it.

### Example 1

```
names = ["mia", "leo", "ana", "zoe"], times = [312, 298, 312, 305]
Output: ["leo", "zoe", "ana", "mia"]
```

"ana" and "mia" share 312 seconds, so "ana" comes first.

### Example 2

```
names = ["joe", "ben", "eli", "kim"], times = [103, 103, 103, 101]
Output: ["kim", "ben", "eli", "joe"]
```

### Example 3

```
names = ["ben", "lea", "fay", "cai"], times = [415, 357, 317, 136]
Output: ["cai", "fay", "lea", "ben"]
```

### Constraints

- 1 ≤ names.length ≤ 10⁴
- times.length = names.length
- Each name has 1 to 10 lowercase English letters or digits, and all names are different.
- 1 ≤ times[i] ≤ 10⁶
