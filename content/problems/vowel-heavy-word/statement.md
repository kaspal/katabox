Lena is building a word game and needs to know the most vowels any word in a list contains. Her function was quick to write, but the numbers it reports rarely match what she counts on paper.

Given an array `words` of lowercase words, return the largest number of vowels found in any single word. The vowels are `a`, `e`, `i`, `o` and `u` (`y` is not a vowel).

The editor starts with that code. Find the bug and fix it.

### Example 1

```
words = ["queue", "sky", "audio", "tree"]
Output: 4
```

"queue" and "audio" both have 4 vowels; "sky" has none.

### Example 2

```
words = ["bebuuuodbu", "o", "auedbia", "a"]
Output: 6
```

### Example 3

```
words = ["bi", "eed", "acooiueaai", "iooccdda"]
Output: 9
```

### Constraints

- 1 ≤ words.length ≤ 10⁴
- Each word has 1 to 20 lowercase English letters.
