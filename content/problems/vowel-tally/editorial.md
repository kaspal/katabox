## Approach: count while scanning

Walk through the text once. For each character, turn it into lowercase and check whether it is in `"aeiou"`; if so, add one to the counter. Digits, spaces and punctuation are never vowels, and lowercasing leaves them unchanged.

**Complexity:** O(n) time, O(1) extra space.
