## Approach: precompute every prefix's answer with a set

Counting the distinct ids of `scans[0 .. p]` from scratch costs O(p) per question, so O(N·Q) in total: hundreds of millions of set insertions at the limits. But every question asks about a **prefix** of the same list, and each prefix is the previous one plus one scan.

Walk the scans once, keeping a set `seen` and an array `distinct` with `distinct[0] = 0`:

```
for p in 1..N:
    add scans[p-1] to seen
    distinct[p] = size of seen
```

Scan `p` grows the set by one exactly when its id is new, so `distinct[p]` is the number of distinct ids among the first `p` scans. Each question is then answered with `distinct[p]` in O(1), whatever order the questions come in.

Read all the tokens at once: the ids sit on one line and the questions on many, but split on whitespace they form one list (`N`, the ids, `Q`, the `p` values). Collect the answers and print them once.

**Complexity:** O(N + Q) expected time, O(N) memory.

**Fast input in each language:** `sys.stdin.buffer.read().split()` (Python), `readFileSync(0, "utf8").split(/\s+/)` (TypeScript), `stream_get_contents(STDIN)` with `preg_split` (PHP), `read_to_string` with `split_ascii_whitespace` (Rust).
