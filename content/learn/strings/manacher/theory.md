## The idea

A **palindrome** reads the same both ways. Growing one from every possible center costs `O(n²)` on a text like `aaaa…`. **Manacher's algorithm** reuses what earlier centers already found, so the whole text takes `O(n)`.

First, a bar goes between the letters and around them: `aba` becomes `|a|b|a|`. Now every palindrome has a center in `s`: odd ones on a letter, even ones on a bar, so one rule handles both. `radius[i]` is how far the palindrome centered at `i` reaches on each side. Thanks to the bars, it is also the length of that palindrome in the original text, and it starts at `(i - radius[i]) / 2`.

The algorithm keeps the palindrome that reaches furthest right so far: its `center` and its edge `right`. For a position `i` inside it, the palindrome mirrors `i` onto `2 · center - i`, whose radius is known. Start from `min(right - i, radius[mirror])`: the mirror's value, cut at the edge, because beyond `right` nothing has been checked. Then **grow** while the letters on both sides agree, and move `center` and `right` if the new palindrome reaches further. The output is the length and the start of the longest palindrome; on a tie, the first one wins. The text is typed as one word, letters a to z.

### The default input

Text `abacabadab`, so `s = |a|b|a|c|a|b|a|d|a|b|` with 21 places. The `b` at `s[3]` grows to radius 3 (`aba`). The `c` at `s[7]` grows all the way to radius 7: `abacaba`, reaching the edge at 14. At `s[11]`, the second `b` of that palindrome, the mirror is `s[3]`, so it starts from `min(14 - 11, 3) = 3`; the next pair, `c` and `d`, differs, and growing stops at once. Later the `d` at `s[15]` grows to 5 (`badab`), still shorter. The output is `[7, 0]`.

## Steps

1. Build `s` with bars, and a `radius` array of zeros; `center = right = 0`.
2. For each `i`: if `i < right`, start from `min(right - i, radius[2 · center - i])`.
3. Grow while both neighbours exist and are equal.
4. If `i + radius[i] > right`, set `center = i` and `right = i + radius[i]`.
5. Pick the first `i` with the largest radius; return `[radius[i], (i - radius[i]) / 2]`.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(n)` |
| Extra space | `O(n)` for `s` and `radius` |

Why linear? If the mirror's palindrome ends strictly inside the current one, it cannot grow: the first new pair is a mirror of a pair that already failed. Otherwise every successful grow step pushes `right` one place further, and `right` never moves back, so there are at most `2n + 1` of them in total.

## Pitfalls

- **Dropping the outer bars.** The bars make even and odd palindromes one case, but only with a bar at both ends; without them, the radius no longer equals the length for palindromes that touch the text's edges.
- **Not cutting at `right`.** The mirror's radius may reach beyond the known part; take the minimum.
- **Reading positions in `s` as positions in the text.** Divide by 2 to convert back.

## When to use it

Use it to find the longest palindromic piece of a text, or to know, for every center, the palindrome around it: counting palindromic substrings is a sum over the radius array. The linked problem, Longest Mirror, asks for exactly this.

## Related topics

- [Z-function](learn:strings/z-function): the same reuse of the furthest box, for prefixes.
- [Knuth-Morris-Pratt](learn:strings/kmp): another linear scan that never repeats work.
- [Two pointers](learn:searching/two-pointers): growing from a center with two moving ends.
