## Idea

Undos come back in reverse order, so record merges on a stack. Use union-find with union by size but **without** path compression: then a merge changes exactly two things, the parent of the absorbed root `rb` and the size of the root it joins, and undoing it means restoring those two. Push `rb` for a real merge and a marker (such as −1) for a merge inside one club, so that undos stay in step with merges. On `[0]`, pop: for a real merge, the root it joined is still `parent[rb]`, because every later merge has already been undone; subtract `size[rb]` from it, set `parent[rb] = rb` and add 1 to the number of clubs. An empty stack means there is nothing to undo. Union by size keeps every tree at most log₂ n tall, so `find` stays fast without compression.

## Complexity

O(log n) per merge (the two `find` calls) and O(1) per undo: O(q log n) time in total, with O(n + q) extra memory.
