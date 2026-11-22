Mateo is analyzing bird songs written as strings of letters, one letter per note type, and wants the longest stretch of a song that uses at most `k` different notes. His function returns believable numbers, but he is not sure it is right on every song.

Given `text` and `k`, return the length of the longest substring (a contiguous part) of `text` that contains at most `k` distinct letters.

The editor starts with that code. Find the bug and fix it.

### Example 1

```
text = "abaccc", k = 2
Output: 4
```

"accc" uses only the letters a and c.

### Example 2

```
text = "bebhhhgdb", k = 2
Output: 4
```

### Example 3

```
text = "abaccbbca", k = 3
Output: 9
```

The whole string uses three letters.

### Constraints

- 1 ≤ text.length ≤ 10⁵
- text consists of lowercase English letters.
- 1 ≤ k ≤ 26
