Counting the three kinds and rewriting the belt takes two passes. Can you keep the belt split into regions as you go?
---
Keep three boundaries: everything before `low` is glass, everything after `high` is plastic, and everything between `low` and `mid` is paper. The part from `mid` to `high` is still unsorted.
---
Look at `items[mid]`: glass → swap it with `items[low]`, move `low` and `mid` on; paper → move `mid` on; plastic → swap it with `items[high]` and move `high` back (do not move `mid`: the item swapped in has not been looked at yet).
