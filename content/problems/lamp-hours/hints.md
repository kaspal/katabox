Adding up every lamp's length counts the overlapping seconds twice. Marking every second as lit or dark works, but there are up to 10⁹ seconds.
---
Sort the lamps by their start time. Then lamps that overlap (or touch) come one after another, and you can join them into lit stretches.
---
Walk through the sorted lamps keeping the current stretch `[s, e)`. If the next lamp starts at or before `e`, extend `e` to the larger end; otherwise add `e − s` to the total and start a new stretch. Do not forget the last stretch.
