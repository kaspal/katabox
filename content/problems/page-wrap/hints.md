The layout is decided word by word, left to right: you only need to know how long the current line already is.
---
Keep the current line's words and its length. A new word fits when `length + 1 + len(word) <= width` (the 1 is the space); the first word of a line always goes in, however long it is.
---
When a word does not fit, join the current words with single spaces, add that line to the result and start a new line with the word. At the end, add the last line.
