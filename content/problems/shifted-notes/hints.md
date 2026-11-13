Do not split the text into words: spaces, punctuation and empty lines must come out exactly where they were. Read the first line as `s`, then keep everything after it as one block of text and change only its letters.
---
A lowercase letter `c` has the position `p = c - 'a'` (0 to 25). Moving it back `s` places gives the position `(p - s + 26) mod 26`; add `'a'` back to get the letter. Uppercase letters work the same with `'A'`.
---
Walk the block character by character, decode letters with the formula above and copy everything else (line breaks included) unchanged. Then print the whole block with one write. A translation table built once (`bytes.maketrans`, `strtr`) does the same walk in a single call.
