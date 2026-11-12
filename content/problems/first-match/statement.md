A crossword helper keeps a word list. When the solver types the first letters of an answer, the helper suggests **the alphabetically first word** in the list that starts with those letters (a word starts with itself). If no word starts with them, it suggests nothing.

`words` is the word list (it may contain repeats) and `prefixes` lists what was typed. Return the suggestion for each prefix, or `""` when there is none.

### Example 1

```
words = ["pear", "peach", "plum", "apple"], prefixes = ["pe", "p", "a", "b", "peac"]
Output: ["peach", "peach", "apple", "", "peach"]
```

"peach" comes before "pear" and "plum" alphabetically.

### Example 2

```
words = ["tree"], prefixes = ["tree", "trees", "t"]
Output: ["tree", "", "tree"]
```

### Example 3

```
words = ["ab", "a", "abc"], prefixes = ["a", "ab"]
Output: ["a", "ab"]
```

A shorter word comes before the longer words it starts.

### Constraints

- 1 ≤ words.length ≤ 10⁴, 1 ≤ prefixes.length ≤ 2·10⁴
- 1 ≤ words[i].length, prefixes[j].length ≤ 20
- Only lowercase English letters
