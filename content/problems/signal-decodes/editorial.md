## Approach: count readings of each prefix

A recursive search that tries "take one digit" and "take two digits" at every position is correct, but it visits every reading one by one, and a message of `n` ones has `Fib(n+1)` readings: far too many for 10⁵ digits.

Count instead. Let `ways[i]` be the number of readings of the first `i` digits `d1 … di`, with `ways[0] = 1` (the empty prefix has one reading: nothing). Every reading of `d1 … di` ends with exactly one of:

- the letter `di` alone: possible when `di ≠ 0`, and the rest is any reading of the first `i-1` digits;
- the letter `d(i-1) di`: possible when those two digits form a number from 10 to 26 (so `d(i-1)` is `1`, or `2` with `di ≤ 6`), and the rest is any reading of the first `i-2` digits.

The two cases never overlap (the last letter has a different length), so

```
ways[i] = (di ≠ 0 ? ways[i-1] : 0) + (10 ≤ d(i-1)di ≤ 26 ? ways[i-2] : 0)
```

Zeros take care of themselves: a `0` contributes only through the pair rule, so `30`, `00` or a leading `0` make the count 0 from there on. The answer is `ways[n]`, taken modulo 1 000 003 at every step (a sum of two residues stays far below 2⁵³). Only the last two values are needed, so two variables replace the array.

**Input handling:** split stdin on whitespace; each message token starts with `#`, so read its digits from the second character. (The `#` also keeps a 10⁵-digit message from looking like a number.)

**Complexity:** O(total digits) time, O(1) extra memory per message.

**Fast input in each language:** `sys.stdin.buffer.read().split()` (Python), `readFileSync(0, "utf8").split(/\s+/)` with `charCodeAt` (TypeScript), `stream_get_contents(STDIN)` with `preg_split` (PHP), `read_to_string` with `split_ascii_whitespace` and `as_bytes` (Rust).
