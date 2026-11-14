## Approach: one fast read, one pass

**Reading.** The log has up to 6·10⁴ lines. Read all of stdin in a single call and split it on whitespace; then convert the tokens to integers.

Why not `input()` once per line in Python? It works, but it is slower: every call is a full trip through the interpreter. It looks up `sys.stdin` and `sys.stdout`, flushes the output, asks the text layer for the next line, decodes it into a new string object and strips the newline. That costs around a microsecond per line, against a few nanoseconds per line for `sys.stdin.buffer.read().split()`, which does the whole job in C. Here that is a few tens of milliseconds. With hundreds of thousands or millions of lines, it becomes a real share of the time limit. The same holds for one `read_line` into a fresh `String` per line in Rust, or `fgets` in PHP: correct, but each call pays a lock, a buffer check and an allocation. Reading everything at once is the idiom to keep for large inputs.

**Computing.** Walk the readings once and update all three numbers in constant time per reading:

1. `total += x`;
2. if `x > prev`, `rises += 1`;
3. if `x == prev`, `run += 1`, otherwise `run = 1`; then `longest = max(longest, run)`.

Start from the first reading with `total = x`, `rises = 0`, `run = longest = 1`.

**Why it is correct:** every rise is a pair of neighbours, counted when the second one arrives; a run of equal readings is extended exactly while the next reading equals the previous one, so `longest` sees the full length of every run.

The sum can reach 6·10⁴ · 10⁶ = 6·10¹⁰: it fits in a 64-bit integer, and in a JavaScript number exactly.

**Complexity:** O(N) time; O(1) extra memory besides the tokens.

**Fast input in each language:** `sys.stdin.buffer.read().split()` (Python), `readFileSync(0, "utf8").split(/\s+/)` (TypeScript), `stream_get_contents(STDIN)` with `preg_split` (PHP), `read_to_string` with `split_ascii_whitespace` (Rust).
