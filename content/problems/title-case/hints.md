A character starts a word when it is not a space and the character before it is a space (or it is the first character).
---
Build the result one character at a time, remembering whether the previous character was a space.
---
Uppercase a character that starts a word, lowercase every other non-space character, and copy spaces as they are. Digits are unchanged either way.
