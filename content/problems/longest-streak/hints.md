Where in the array is the longest run in the inputs that fail? Try `[1, 2, 2]` and a single-element array like `[5]`.
---
The run length is counted correctly. Look at when `best` gets updated, and what happens to the last run.
---
`best` is only updated when a run is broken by a different value, so the final run is never considered. After the loop, return `max(best, run)` instead of `best`.
