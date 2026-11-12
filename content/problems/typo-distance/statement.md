A spell checker ranks its suggestions by how many keystroke fixes separate what you `typed` from the word you `meant`. One fix is one of:

- **insert** a letter anywhere,
- **delete** a letter,
- **replace** a letter with another.

Return the fewest fixes that turn `typed` into `meant`.

### Example 1

```
typed = "cofee", meant = "coffee"
Output: 1
```

Insert one `f`.

### Example 2

```
typed = "stake", meant = "skate"
Output: 2
```

Replace the `t` with `k` and the `k` with `t`.

### Example 3

```
typed = "lamp", meant = "pal"
Output: 3
```

One way: replace `l` with `p` (`pamp`), delete `m` (`pap`), replace the last `p` with `l` (`pal`).

### Constraints

- 1 ≤ typed.length, meant.length ≤ 1000
- typed and meant contain only lowercase English letters.
