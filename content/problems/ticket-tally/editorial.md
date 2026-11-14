## Approach: read tokens, walk with a position

The input mixes counts and values over many lines, and some lines are empty. Reading it line by line means handling each line's shape. Reading **everything at once and splitting on whitespace** turns it into one flat list of tokens, where an empty line contributes nothing:

```
2 3 12 30 8 1 45
```

Keep an index `pos` into that list:

1. `T = tokens[0]`, `pos = 1`.
2. For each day: `n = tokens[pos]`, then the prices are `tokens[pos+1 .. pos+n]`; move `pos` past them.
3. Print `n`, the sum and the maximum (0 when `n` is 0).

Build the output lines in a list and print them once: one write is much faster than one per line.

**Complexity:** O(total input) time and memory.

**Fast input in each language:** `sys.stdin.buffer.read().split()` (Python), `readFileSync(0, "utf8").split(/\s+/)` (TypeScript), `stream_get_contents(STDIN)` with `preg_split` (PHP), `read_to_string` with `split_ascii_whitespace` (Rust).
