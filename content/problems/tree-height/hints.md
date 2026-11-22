Start with the smallest trees: what does the code return for `[]` and for a single node `[7]`? Are the answers off for every tree, or only some?
---
The recursive step "one for this node plus the taller subtree" is fine. Look at what the function returns when there is no node at all.
---
The base case returns 1 for a missing node, so every path gets one phantom level at the bottom. An empty tree has height 0: return 0 when `root` is `None` (`null` in TypeScript).
