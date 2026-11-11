Walk through `text` once. How do you know that a run has just ended?
---
A run ends when the next letter is different, or when you reach the end of the text. Keep the letter of the current run and its length so far.
---
When a run ends, append its letter, then its length if it is at least 2. Build the result in a list (or a string builder) and join it once: repeated string concatenation can be slow.
