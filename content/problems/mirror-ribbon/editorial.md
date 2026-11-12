## Approach: two pointers that skip symbols

Use one index `i` from the left and one index `j` from the right.

1. Move `i` right while `text[i]` is not a letter or digit, and `j` left in the same way (never letting them cross).
2. If `i < j`, compare `text[i]` and `text[j]` in lower case. Different characters mean the ribbon is not a mirror.
3. Otherwise step both inwards and repeat.

When the indices meet, every pair matched.

### Walkthrough

`text = "Top spot: 7 -- 7 tops pot"` compares `T`/`t`, `o`/`o`, `p`/`p`, `s`/`s`, … and finally `7`/`7`; the spaces, the colon and the dashes are skipped.

**Complexity:** O(n) time and O(1) extra space. Building a cleaned copy and its reverse is also O(n), but needs O(n) extra memory.
