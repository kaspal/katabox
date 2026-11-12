A tiny e-reader shows `width` characters per line. It lays out a text, given as a list of `words`, like this:

- Words go onto the current line, separated by **one space**, as long as the line stays at most `width` characters long.
- When the next word does not fit, the current line ends and the word starts a new line.
- A word longer than `width` is never split: it gets a line of its own (the only case where a line is longer than `width`).

Return the lines, in order, without spaces at their ends.

### Example 1

```
words = ["the", "quick", "brown", "fox", "jumps"], width = 10
Output: ["the quick", "brown fox", "jumps"]
```

"the quick" has 9 characters; adding " brown" would make 15.

### Example 2

```
words = ["encyclopedia", "is", "big"], width = 5
Output: ["encyclopedia", "is", "big"]
```

"is big" would need 6 characters.

### Example 3

```
words = ["a", "b", "c"], width = 5
Output: ["a b c"]
```

### Constraints

- 1 ≤ words.length ≤ 10⁴
- 1 ≤ words[i].length ≤ 20, lowercase English letters
- 1 ≤ width ≤ 100
