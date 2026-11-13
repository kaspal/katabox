Split the whole input on whitespace. The map rows have no spaces, so each row is exactly one token; a question is two tokens, the word and the index.
---
Counting a row or a column for every question costs up to 300 steps, and there can be 2·10⁴ questions. The map never changes, so count every row and every column once, before the questions.
---
Make two arrays, `rowTrees[R]` and `colTrees[C]`. For every `#` at row `i`, column `j`, add 1 to `rowTrees[i]` and `colTrees[j]`. Then each question is one lookup: `row r` prints `rowTrees[r]`, `col c` prints `colTrees[c]`.
