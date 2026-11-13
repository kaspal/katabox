## Approach: count once, answer by lookup

**Reading the mixed lines.** The input has three kinds of lines: two numbers, a row of map characters, and a word with a number. Split the whole input on whitespace and walk the tokens with a position:

1. `R`, `C`;
2. the next `R` tokens are the map rows (a row never contains a space, so it is a single token);
3. `Q`;
4. then `Q` pairs `kind index`.

**Precomputing.** Answering each question by scanning its row or column costs up to 300 steps, so up to 6·10⁶ steps in all: acceptable in a compiled language, slow in an interpreted one, and wasted work either way, since the map never changes. Instead, scan the map once: for every `#` at `(i, j)`, add 1 to `rowTrees[i]` and to `colTrees[j]`. Each question is then a single array lookup.

**Why it is correct:** every tree is counted once in its row's total and once in its column's total, which is exactly what each question asks.

**Complexity:** O(R·C + Q) time, O(R + C) extra memory.

**Fast input in each language:** `sys.stdin.buffer.read().split()` (Python; `row.count(b"#")` counts a row, and `zip(*rows)` gives the columns), `readFileSync(0, "utf8").split(/\s+/)` with `charCodeAt` (TypeScript), `stream_get_contents(STDIN)` with `preg_split` and `substr_count` (PHP), `read_to_string` with `split_ascii_whitespace`, `as_bytes()` and a `BufWriter` (Rust). Collect the answers and write them once.
