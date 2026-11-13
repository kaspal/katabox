A library's catalogue has book titles typed in every possible mix of capitals. Fix a title so that every **word** starts with an uppercase letter and the rest of the word is lowercase.

A word is a maximal run of non-space characters. Spaces stay exactly where they are, including repeated spaces. A word that starts with a digit keeps it, and its letters become lowercase.

Return the fixed title.

### Example 1

```
text = "the old MAN and the sea"
Output: "The Old Man And The Sea"
```

### Example 2

```
text = "  two  spaces "
Output: "  Two  Spaces "
```

### Example 3

```
text = "route 66 rules"
Output: "Route 66 Rules"
```

### Constraints

- 0 ≤ text.length ≤ 10⁴
- text has English letters, digits and spaces only.
