## Approach: longest palindromic prefix with the prefix function

**What the answer looks like.** Say we add `k` letters `x` in front, so the result is `p = x + s`, with `n = |s|`. If `p` is a palindrome, its first `k` letters mirror its last `k` letters, so `x` is the last `k` letters of `s`, reversed. What lies between them, the first `n − k` letters of `s`, must then mirror itself: it is a palindrome. Conversely, if `s[0 : n − k]` is a palindrome, then `reverse(s[n − k:]) + s` is one. So the fewest letters to add means the **longest palindromic prefix** of `s`, of length `keep`, and the answer is

```
reverse(s[keep:]) + s
```

For `abac` the palindromic prefixes are `a` and `aba`, so `keep = 3` and the answer is `c` + `abac` = `cabac`.

**Finding the longest palindromic prefix.** A prefix of length `L` is a palindrome exactly when it equals its reverse, and its reverse is the suffix of length `L` of `reverse(s)`. So we want the longest prefix of `s` that is also a suffix of `reverse(s)`. That is what the prefix function of KMP measures: for a string `t`, `pi[i]` is the length of the longest proper prefix of `t[0..i]` that is also a suffix of it. Build

```
t = s + "#" + reverse(s)
```

and compute `pi` in one pass. At the last position, `pi` is the longest prefix of `t` that ends the whole of `t`. Since `#` appears only once and never in `s`, such a match cannot run across the middle, so it has length at most `n`: it is a prefix of `s` that is a suffix of `reverse(s)`. That length is `keep`.

For `abac`, `t = abac#caba` and its prefix function ends with `3` (`aba`).

**Why it is correct:** the first paragraph shows that every palindrome ending with `s` adds at least `n − keep` letters and that `n − keep` letters are enough; the prefix function finds `keep` exactly. The answer is unique, because the `k` added letters are forced.

**Complexity:** O(n) time and memory. The prefix function's inner `while` only goes back as far as earlier steps went forward, so the whole pass is linear.

**The obvious approach is too slow.** Testing prefixes from the longest down, letter by letter from both ends, is O(n²) in the worst case. A word like `a…a b a…a`, with twice as many `a`s after the `b`, makes many prefixes nearly palindromes: at 2·10⁵ letters that is about 4.4·10⁹ letter comparisons.

**Practical notes:** read the whole input at once and drop the line break (`trim`, `split`); a stray `\r` would otherwise end up inside the word. Write the answer, up to 4·10⁵ − 1 letters, in one write. In TypeScript compare with `charCodeAt` and keep `pi` in an `Int32Array`; in Rust work on the bytes (`as_bytes()`), since all letters are ASCII.
