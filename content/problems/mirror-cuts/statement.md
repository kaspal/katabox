A banner maker sells strips of letters that read the same forwards and backwards (**palindromes**). A customer wants the banner `text`, so it must be cut into pieces that are each a palindrome; a single letter always is.

Return the smallest number of cuts needed.

### Example 1

```
text = "noonabc"
Output: 3
```

"noon" | "a" | "b" | "c".

### Example 2

```
text = "racecar"
Output: 0
```

### Example 3

```
text = "abab"
Output: 1
```

"aba" | "b".

### Constraints

- 1 ≤ text.length ≤ 1500
- text has lowercase English letters only.
