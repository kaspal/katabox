A sign painter has been asked to turn a shop's name into a palindrome, a word that reads the same from both ends. The name is already painted on the board and cannot be touched, and there is free space only to its left, so the painter can only add letters in front of it. Paint is expensive: the fewer letters, the better.

This is a **program problem**: read the word from standard input and print the shortest palindrome that ends with it to standard output.

## Input format

- One line holds the word `s`, in lowercase letters.

## Output format

Print one line: the shortest palindrome that is made of some letters followed by `s`. If `s` is already a palindrome, print it unchanged.

### Example 1

Input:

```
abac
```

Output:

```
cabac
```

`abac` starts with the palindrome `aba`, so one `c` in front is enough. No shorter word works: `abac` itself is not a palindrome.

### Example 2

Input:

```
racecar
```

Output:

```
racecar
```

### Example 3

Input:

```
ab
```

Output:

```
bab
```

### Constraints

- 1 ≤ |s| ≤ 2·10⁵
- `s` contains only the lowercase letters `a` to `z`.
