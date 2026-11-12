## Approach: a map and two stacks of change records

Keep `values`, a hash map from key to current value, and two stacks of **change records** `(key, old, new)`, where `old` is the value before the write (or "none").

- `write(k, v)`: push `(k, values.get(k), v)` onto `undo`, set `values[k] = v`, and clear `redo`.
- `read(k)`: report `values.get(k, -1)`.
- `undo`: pop `(k, old, new)` from `undo`; restore `old` (delete the key if `old` is "none"); push the record onto `redo`.
- `redo`: pop `(k, old, new)` from `redo`; set `values[k] = new`; push the record onto `undo`.

Undo restores exactly the state before the write it takes back, because later writes have already been undone (stack order). So every record's `old` is correct when it is used. Clearing `redo` on a new write is what the statement asks: the undone future no longer applies.

### Walkthrough

Example 3: write (1, 5) pushes (1, none, 5); undo removes key 1 and moves the record to redo; write (2, 6) clears redo; redo does nothing. Reads: key 1 → -1, key 2 → 6.

**Complexity:** O(1) expected per operation and O(ops) memory. Replaying the whole history of writes for every read is O(ops) per read, O(ops²) in all.
