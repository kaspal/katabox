## Approach: sort, remove duplicates, look up

For a score `x` that someone got, its rank is **1 + the number of distinct scores greater than `x`**. Computing that for each question by scanning all `N` scores (and collecting the greater ones in a set) costs O(N) per question, O(N·Q) in total: hundreds of millions of steps at the limits.

Do the work once instead:

1. Take the distinct scores and sort them from highest to lowest: `100 85 70 60` in Example 1.
2. The score at index `j` (from 0) of that list has exactly `j` distinct scores above it, so its rank is `j + 1`.
3. For each asked score, find it in the list. Either binary-search it (the list is sorted), or build a hash map `score → rank` once and look it up. If it is not there, nobody got that score: print `0`.

Equal scores share a rank because duplicates were removed before numbering: in Example 1 both 85s are rank 2, and 70 is rank 3, not 4.

Read all the tokens at once (`N`, the scores, `Q`, the asked scores) and print the answers joined by spaces on a single line.

**Complexity:** O(N log N + Q log N) time with binary search (O(N log N + Q) expected with a map), O(N) memory.

**Fast input in each language:** `sys.stdin.buffer.read().split()` (Python), `readFileSync(0, "utf8").split(/\s+/)` (TypeScript), `stream_get_contents(STDIN)` with `preg_split` (PHP), `read_to_string` with `split_ascii_whitespace` (Rust).
