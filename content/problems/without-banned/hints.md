Try two banned values right next to each other, such as `items = [7, 7]` with `banned = [7]`. How many survive?
---
The membership test is fine. Think about what happens to the loop when the list it is walking over changes under it.
---
Removing an element shifts everything after it one place left, so the loop skips the element that moves into the current position. Do not modify `items` while iterating; build a new list of the allowed values instead (in TypeScript, use `filter`).
