## Approach: a trie of the dictionary, one DFS per cell

1. **Trie:** insert every word; the node where a word ends stores it.
2. **Search:** from every cell, run a depth-first search that walks the grid and the trie together. Entering a cell is allowed only if its letter is a child of the current trie node and the cell is not already on the path. When the new trie node stores a word, record it.
3. **Backtrack:** unmark the cell when returning.
4. Sort the recorded words.

Every word with the same prefix is checked by the same walk, so shared beginnings are explored once. Pruning by the trie stops a path as soon as no word continues it.

**Complexity:** at most cells · 4 · 3^(L−1) path steps for words of length ≤ L, independent of the number of words; plus O(total length) to build the trie. Searching for each word separately repeats that work for every word: with 3·10⁴ words that share long prefixes and never match, it is about 10⁹-10¹⁰ steps.
