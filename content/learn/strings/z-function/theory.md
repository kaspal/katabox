## The idea

Glue the pattern and the text together with a bar between them: `pattern|text`. For every position `i` of this string `s`, `z[i]` is the length of the longest piece starting at `i` that agrees with the start of `s`. The bar is not a letter, so no agreement can run past it: `z[i]` is at most `m`, the pattern length, and a position in the text with `z[i] = m` is a full match. Its start in the text is `i - m - 1`.

Computing each `z[i]` by comparing from scratch can cost `O(n · m)`. The trick is the **Z-box** `[left, right)`: the agreement that reaches furthest right so far. Inside it, `s[left..right)` is a copy of `s[0..right - left)`. So a position `i` inside the box looks like position `i - left` near the start, whose value is already known. Start from `min(right - i, z[i - left])`: the mirrored value, cut at the box edge, because nothing past `right` has been checked yet. Then **extend** letter by letter, and move the box if the agreement now reaches further right. `z[0]` is left at 0; it is never read. The output is the count of matches, then their positions. The text and the pattern are typed as two words, letters a to z.

### The default input

Text `aabaabaab`, pattern `aab`, so `s = aab|aabaabaab`. `z[1] = 1` (`a`, then `b` differs from `a`) and `z[4] = 3`: `aab`, then the bar stops it. The box becomes `[4, 7)`. At `i = 5`, the box copies `z[1] = 1`, and one more comparison fails. At `i = 6`, it copies `z[2] = 0`. At `i = 7`, outside the box, extending gives 3 again, and the same happens at `i = 10`. The positions with `z = 3` are 4, 7 and 10, so the output is `[3, 0, 3, 6]`.

## Steps

1. Build `s = pattern + "|" + text` and a `z` array of zeros, with `left = right = 0`.
2. For each `i` from 1: if `i < right`, start from `min(right - i, z[i - left])`.
3. While `s[z[i]]` equals `s[i + z[i]]`, increase `z[i]`.
4. If `i + z[i] > right`, set `left = i` and `right = i + z[i]`.
5. Every `i` after the bar with `z[i] = m` is a match at `i - m - 1`.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(n + m)`: each successful extension pushes `right` forward |
| Extra space | `O(n + m)` for `s` and `z` |

A comparison that succeeds while extending moves `right` one place further, and `right` never moves back, so there are at most `n + m` of them; each position adds at most one failed comparison.

## Pitfalls

- **A separator that can appear in the text.** Then agreements could cross it and `z` could exceed `m`. Any symbol that is not a letter works.
- **Not cutting at the box edge.** The mirrored value may run past `right`, where nothing is known; take the minimum.
- **Exclusive right end.** Here `right` is one past the box, so the test is `i < right`.

## When to use it

Use it for single-pattern search, or for questions about prefixes: how far each position agrees with the start, the shortest period of a word. Compared with [Knuth-Morris-Pratt](learn:strings/kmp), it finds the same matches in the same time, with one array computed over pattern and text together instead of a table plus a scan. The linked problem, Pattern Count, works with either.

## Related topics

- [Knuth-Morris-Pratt](learn:strings/kmp): borders instead of prefix agreements.
- [Manacher's longest palindrome](learn:strings/manacher): the same reuse of a box, around palindromes.
- [Rabin-Karp](learn:strings/rabin-karp): search by hashing windows.
