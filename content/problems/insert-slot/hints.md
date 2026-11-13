The list is already sorted and clean. Which slots can the new one possibly touch?
---
The slots split into three groups: those that end before `fresh` starts (untouched, on the left), those that share a moment with it, and those that start after it ends (untouched, on the right).
---
Copy the left group, then absorb every slot with `start ≤ fresh end` into `fresh` (take the smaller start and the larger end), add the grown slot, and copy the rest.
