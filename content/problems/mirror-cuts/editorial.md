## Approach: cut DP with palindromes found by expanding centres

Let `cuts[i]` = fewest cuts for the prefix of length `i`, with `cuts[0] = −1` (so that a prefix that is itself a palindrome gets `−1 + 1 = 0`) and `cuts[i] = i − 1` at first (cutting between every pair of letters).

Every palindrome has a centre: a letter (odd length) or a gap (even length). Expanding from a centre while `text[l] == text[r]` lists every palindrome `text[l..r]` around it; each one can end a split of `text[0..r]`:

```
cuts[r + 1] = min(cuts[r + 1], cuts[l] + 1)
```

Process the centres from left to right. When the centre is at position c, every palindrome starting at l ends at or after c, so `cuts[l]` (with l ≤ c) only depends on palindromes ending before c, which have already been seen. The answer is `cuts[n]`.

**Complexity:** O(n²) time (at most about 10⁶ expansions here), O(n) space. Trying every split recursively without remembering results is exponential.
