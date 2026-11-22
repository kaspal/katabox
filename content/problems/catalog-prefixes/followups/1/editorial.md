## Idea

In the base problem the trie was built once; here it changes, and per-node counters make that easy. Each node keeps `count`, the number of catalogue titles whose path goes through it, which is exactly the number of titles starting with the prefix the node spells. `"add"` walks the title's letters, creating nodes as needed, and adds 1 to every node on the path; `"remove"` walks the same path and subtracts 1 (the title is guaranteed to be there, so no counter goes below 0). `"count"` follows the typed letters and returns the last node's counter, or 0 if some letter has no child. Nodes whose counter drops to 0 can stay: they simply report 0.

## Complexity

O(L) per operation, where L ≤ 20 is the word length: O(q · L) time in total, and at most O(q · L) trie nodes.
