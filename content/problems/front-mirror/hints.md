If you add `k` letters in front, which letters must they be? Compare the first `k` letters of the result with its last `k` letters. What must then be true of the rest of `s`?
---
The added letters are the last letters of `s`, reversed, and what remains at the start of `s` must be a palindrome. So the task is to find the **longest palindrome at the start of `s`**. Checking each prefix from the longest down can take about `n²` steps; a string-matching tool finds it in one pass.
---
A prefix of `s` is a palindrome exactly when it equals a suffix of `reverse(s)`. Build `t = s + "#" + reverse(s)` and compute its prefix function (KMP): the value at the last position, `keep`, is the length of the longest palindromic prefix (`#` stops a match from crossing the middle). Print `reverse(s[keep:]) + s`.
