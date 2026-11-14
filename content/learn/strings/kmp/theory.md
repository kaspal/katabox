## The idea

To find a pattern in a text, the obvious way tries every start and compares letter by letter, going back to the next start after each mismatch. That can cost `O(n · m)`. **Knuth-Morris-Pratt** never goes back in the text: when a mismatch comes, the pattern itself says how much of what was already matched can be kept.

The key is a **border**: a piece that is both a proper prefix and a suffix of a word. `abab` has the border `ab`. The **failure table** stores, for each prefix of the pattern, the length of its longest proper border. It is built by running the same matching idea on the pattern against itself: `k` is the current border length; on a mismatch, `k` falls to `fail[k - 1]`, on a match it grows by one.

The scan then reads the text once. `k` counts how many pattern letters match right now. On a mismatch, `k` drops to `fail[k - 1]`: the border of the matched part is already known to match, so it is kept. When `k` reaches `m`, a match ends here; record its start, `i - m + 1`, and slide to the border again, so **overlapping** matches are found too. The output is the count, then the start positions. The text and the pattern are typed as two words, letters a to z.

### The default input

Text `abababcabab`, pattern `abab`. The failure table is `[0, 0, 1, 2]`: `a` and `ab` have no border, `aba` has `a`, `abab` has `ab`.

The scan matches `abab` at position 0, then keeps `k = 2` and finishes a second match after two more letters, at position 2: the two overlap. At the `c`, the pattern wants an `a`: `k` falls from 2 to `fail[1] = 0`, and `c` does not match either. The scan restarts and finds `abab` at position 7. The output is `[3, 0, 2, 7]`.

## Steps

1. Build `fail`: for each `i` from 1, while `k > 0` and `pattern[i] ≠ pattern[k]`, set `k = fail[k - 1]`; if they match, `k += 1`; store `fail[i] = k`.
2. Scan the text with `k = 0`: fall back the same way on a mismatch, then `k += 1` on a match.
3. When `k = m`, record `i - m + 1` and set `k = fail[m - 1]`.
4. Return the count, then the positions.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(n + m)`: the table, then one pass over the text |
| Extra space | `O(m)` for the failure table |

Each step of the scan either moves to the next letter or makes `k` smaller. `k` grows by at most one per letter, so it cannot fall more than `n` times in total: the inner loop adds `O(n)` overall, not per letter.

## Pitfalls

- **Off by one in the fallback.** A mismatch with `k` letters matched uses `fail[k - 1]`, the border of those `k` letters, not `fail[k]`.
- **Resetting to 0 after a match.** That loses overlapping matches; fall to the border instead.
- **Starting the table at 0.** `fail[0]` is always 0, since a single letter has no proper border; the loop starts at 1, or every letter would match itself.

## When to use it

Use it to count or list every occurrence of one pattern in a long text, with a guaranteed linear time. The linked problem, Pattern Count, asks exactly that. The failure table alone answers questions about borders and periods of a word.

## Related topics

- [Z-function](learn:strings/z-function): another linear search, built from prefix agreements.
- [Rabin-Karp](learn:strings/rabin-karp): the same search with a rolling hash.
- [Aho-Corasick](learn:strings/aho-corasick): failure links for many patterns at once.
- [Sliding window](learn:searching/sliding-window): another single pass that never moves back.
