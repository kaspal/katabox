## Approach: prefix sums of run lengths, then binary search

A label like `1000000000x5y999999999z` is two billion letters long, so building the unpacked string is impossible. Walking the packed text run by run for every check works, but costs O(runs) per check: with 60 000 runs and 20 000 checks that is over 10⁹ steps.

Parse each label **once** into two arrays:

- `letters[r]`, the letter of run `r`;
- `ends[r]`, the position of the last letter of run `r`: the sum of the counts of runs `0 .. r`.

For `3a2b10c`: `letters = [a, b, c]`, `ends = [3, 5, 15]`. Run `r` covers positions `ends[r-1] + 1 .. ends[r]`, so position `k` lies in the **first run with `ends[r] ≥ k`**. Every count is at least 1, so `ends` is strictly increasing and binary search finds that run in O(log runs). In the example, `k = 4` gives `r = 1`, the letter `b`.

Two runs in a row may share a letter (`2k1k3m`); nothing changes, since the answer is just the letter of the run found.

**Input handling:** split all of stdin on whitespace; the tokens are `L`, the `L` labels, `Q`, then `i k` pairs. To parse a label, read digits into a running count and close the run at each letter. Every count is at most 10⁹ and a label has at most 60 000 runs, so every end is below 6·10¹³ < 2⁵³: plain `Number` is exact in TypeScript and `int` in PHP.

**Complexity:** O(total label length + Q · log runs) time, O(total runs) memory.

**Fast input in each language:** `sys.stdin.buffer.read().split()` with `re.findall` and `bisect_left` (Python), `readFileSync(0, "utf8").split(/\s+/)` and `charCodeAt` (TypeScript), `stream_get_contents(STDIN)` with `preg_split` and `preg_match_all` (PHP), `read_to_string` with `split_ascii_whitespace` and `partition_point` (Rust).
