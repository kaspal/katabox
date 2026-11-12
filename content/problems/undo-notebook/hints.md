Undo takes back the most recent write first, and redo re-applies the most recently undone one first: both are "last in, first out".
---
Keep the current values in a hash map, plus two stacks. For each write, push onto the undo stack what is needed to take it back: the key, the value it replaced (or "none") and the new value.
---
`undo` pops from the undo stack, restores the old value and pushes the same record onto the redo stack; `redo` pops from the redo stack, sets the new value again and pushes the record back onto the undo stack. A new `write` empties the redo stack.
