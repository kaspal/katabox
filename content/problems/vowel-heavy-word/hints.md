Put the word with the most vowels first and a word with few vowels last. Does the answer change if you reorder the list?
---
The vowel count for each word is right. Follow the variable that is supposed to remember the best count across words.
---
`best` is reset to `0` inside the loop, so it forgets earlier words (in the TypeScript version an inner `let best` shadows the outer one, which never changes). Initialize `best = 0` once, before the loop.
