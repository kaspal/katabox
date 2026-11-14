At a summer camp, the counselors run a night-time clue hunt. To keep the clues from being read at a glance, they encode them: every letter is moved forward `s` places in the alphabet, wrapping around from `z` to `a` (and from `Z` to `A`). With `s = 3`, `a` becomes `d`, `y` becomes `b` and `M` becomes `P`; digits, spaces and punctuation stay as they are. The camp's head has the key and wants a program that turns a stack of encoded clues back into plain text.

This is a **program problem**: read the key and the encoded text from standard input and print the decoded text to standard output.

## Input format

- The first line holds `s`, the shift that was used to encode the text.
- Every line after it, up to the end of the input, is a line of encoded text. Lines may be empty. The number of lines is not given: read until the input ends.

## Output format

Print every text line decoded, in order: each letter moves **back** `s` places within its own case (wrapping from `a` to `z` and from `A` to `Z`), and every other character is unchanged. Empty lines stay empty lines.

The output is compared line by line, exactly apart from trailing spaces.

### Example 1

Input:

```
3
Phhw dw wkh Rog Rdn dw 5.
Eulqj d odps!
```

Output:

```
Meet at the Old Oak at 5.
Bring a lamp!
```

### Example 2

Input:

```
25
Ydaqz bqnrrhmf, ynmd Z.

Qnnl 42 hr nodm.
```

Output:

```
Zebra crossing, zone A.

Room 42 is open.
```

Moving back 25 places is the same as moving forward 1. The empty line stays in place.

### Example 3

Input:

```
0
No secrets here: 100% plain!
```

Output:

```
No secrets here: 100% plain!
```

### Constraints

- 0 ≤ s ≤ 25
- There is at least one text line.
- The text after the first line has at most 150 000 characters, each a printable ASCII character (letters, digits, spaces, punctuation) or a line break.
