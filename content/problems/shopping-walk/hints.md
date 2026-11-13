Growing a stretch from every starting stall until it covers the list can be O(n²). Once a stretch covers the list, what can you do with its left end?
---
Use a window: extend the right end until the window covers the list, then move the left end forward as long as it still covers. Every covering window gets a chance to be the shortest.
---
Keep `missing`, the number of list items not yet covered by the window. Adding a stall of a letter still needed decreases it; removing one that drops a letter below its need increases it. Record the length whenever `missing = 0`.
