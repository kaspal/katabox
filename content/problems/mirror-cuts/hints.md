Let `cuts[i]` be the answer for the first `i` letters. The last piece of the best split is some palindrome `text[j..i)`.
---
`cuts[i] = min over palindromes text[j..i) of (cuts[j] + 1)`, with `cuts[0] = −1` so that a whole palindrome needs 0 cuts. You need to know quickly which substrings are palindromes.
---
Expand around every centre (each letter and each gap): while `text[l] == text[r]`, the piece `text[l..r]` is a palindrome, so update `cuts[r + 1] = min(cuts[r + 1], cuts[l] + 1)`, then widen. Process centres from left to right.
