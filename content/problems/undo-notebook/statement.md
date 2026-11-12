A lab notebook app stores one number per experiment key and supports undo and redo. Process a list of operations, where `ops[i]` is the operation's name and `args[i]` its numbers:

- `"write"`, args `[key, value]`: set the key's value. This also **clears** everything that could be redone.
- `"read"`, args `[key]`: **report** the key's current value, or `-1` if it has none.
- `"undo"`, args `[]`: take back the most recent write that is still in effect, restoring the key's previous value (or no value). If there is nothing to undo, nothing happens.
- `"redo"`, args `[]`: apply again the most recently undone write. If there is nothing to redo, nothing happens.

Return the reports of the `"read"` operations, in order.

### Example 1

```
ops = ["write", "write", "read", "undo", "read", "redo", "read", "undo", "undo", "read"], args = [[1, 10], [1, 20], [1], [], [1], [], [1], [], [], [1]]
Output: [20, 10, 20, -1]
```

After undoing both writes, key 1 has no value again.

### Example 2

```
ops = ["read", "undo", "redo"], args = [[7], [], []]
Output: [-1]
```

### Example 3

```
ops = ["write", "undo", "write", "redo", "read", "read"], args = [[1, 5], [], [2, 6], [], [1], [2]]
Output: [-1, 6]
```

The second write clears the undone write of key 1, so `redo` has nothing to do.

### Constraints

- 1 ≤ ops.length ≤ 10⁵, args.length = ops.length
- Each `ops[i]` is `"write"`, `"read"`, `"undo"` or `"redo"`, with the numbers described above
- 1 ≤ key ≤ 10⁹, 0 ≤ value ≤ 10⁹
