Comparing every typed text with every title works, but with 2·10⁴ searches over 10⁴ titles that is 2·10⁸ comparisons. Can the titles be stored so that all titles sharing a beginning are found together?
---
Store the titles in a **trie**: a tree where each edge is a letter and each node stands for the prefix spelled on the way from the root. Every title is a path from the root.
---
While inserting a title, add 1 to a counter in every node you pass. The counter of the node for a prefix is then exactly the number of titles beginning with it; a search just walks down the typed letters (0 if a letter is missing).
