The members of a bird-watching club write their field notes into one shared notebook, which is typed up at the end of the season. For the cover of the yearly newsletter, the editor wants the words that appear most often in those notes, with how many times each one appears.

This is a **program problem**: read the notes from standard input and print the most frequent words to standard output.

## Input format

- The first line holds `K`, how many words the editor wants.
- Every line after it, up to the end of the input, is text from the notebook. A **word** is a maximal run of lowercase letters `a`-`z`. Words are separated by spaces and the punctuation marks `.` `,` `;` `:` `!` `?`, in any amount, and a line may be empty. The number of text lines is not given: read until the input ends.

## Output format

Print the `K` most frequent words, one per line as `word count`: the most frequent first, and words with the same count in alphabetical order. If the notes contain fewer than `K` different words, print all of them.

### Example 1

Input:

```
2
the heron saw the duck.
the duck flew!
```

Output:

```
the 3
duck 2
```

### Example 2

Input:

```
3
wren, owl; jay: wren?
owl
```

Output:

```
owl 2
wren 2
jay 1
```

`owl` and `wren` are tied, so they come in alphabetical order.

### Example 3

Input:

```
5
coot coot coot
```

Output:

```
coot 3
```

### Example 4

Input:

```
2
rain,rain!sun

sun...rain
```

Output:

```
rain 3
sun 2
```

Punctuation separates words even without a space, and the empty line holds no words.

### Constraints

- 1 ≤ K ≤ 100
- The text after the first line has at most 200 000 characters, all of them lowercase letters, spaces, line breaks or `.,;:!?`.
- The text holds at least one word.
