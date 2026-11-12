Look at each free seat on its own: it is quiet when the 3 × 3 block around it, clipped to the map, has no `#`.
---
Loop over the row offsets `-1, 0, 1` and the column offsets `-1, 0, 1`, skipping positions outside the map.
---
Each seat checks at most 8 neighbours, so the whole map takes O(rows × columns) time.
