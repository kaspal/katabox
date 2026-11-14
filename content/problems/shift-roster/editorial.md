## Approach: two passes over the parsed roster

The report needs the total first, and deciding who is above the average needs the total too, so read the whole roster before printing anything.

1. Read all of stdin and split it into tokens: `N`, then `name hours` pairs. The names are tokens too, since they never contain spaces.
2. Sum the hours.
3. Walk the roster in input order and keep every volunteer with `hours > total / N`.

**Avoiding fractions.** The average 49 / 5 = 9.8 is not a whole number. Multiplying both sides by `N` (which is positive) gives the equivalent test `hours × N > total`, which uses only integers: no floating-point rounding, and the products stay below 24 · 2·10⁴.

**Output format.** This problem is judged line by line (trailing spaces aside), so the shape of the report matters: the total alone on the first line, one space between a name and its hours, and the word `none` when the list is empty. Collect the lines and write them once.

**Why it is correct:** the test `hours × N > total` is exactly "strictly above the average", and the walk keeps the input order.

**Complexity:** O(N) time and memory.

**Fast input in each language:** `sys.stdin.buffer.read().split()` (Python; decode the name bytes before printing), `readFileSync(0, "utf8").split(/\s+/)` (TypeScript), `stream_get_contents(STDIN)` with `preg_split` (PHP), `read_to_string` with `split_ascii_whitespace` and a `BufWriter` (Rust).
