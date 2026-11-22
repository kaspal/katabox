Scanning every title on each count, or rebuilding everything after each change, is too slow for 3·10⁴ operations. Which structure answered prefix counts in the base problem, and can it change?
---
In a trie, store at every node how many titles pass through it. A count is then just a walk down the typed letters.
---
Adding a title walks down its letters, creating missing nodes, and adds 1 to the counter of every node on the way; removing walks the same path and subtracts 1. A count reads the counter of the node where the typed text ends, or 0 if the path breaks off.
