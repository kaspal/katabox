Searching the grid separately for each word repeats the same work: many words share their beginnings.
---
Put all the words in a trie. Then one depth-first search from each cell can follow the trie and the grid together, checking every word with that prefix at once.
---
DFS(cell, trie node): stop if the cell's letter has no child in the trie; mark the cell as used, record the child's word if it ends there, explore the four neighbours, then unmark. Removing found words from the trie (or marking them) avoids reporting them twice.
