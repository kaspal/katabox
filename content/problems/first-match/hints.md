Scanning the whole list for every prefix works but repeats the same comparisons up to 2·10⁸ times. Words that share a beginning should be found together.
---
Put the words in a **trie**. All words starting with a prefix are exactly the words below the prefix's node.
---
While inserting, store in every node the alphabetically smallest word that passes through it (compare with the stored one, or insert the words in sorted order so the first one to arrive is the smallest). A query walks down the prefix and returns that word, or `""` if the path breaks off.
