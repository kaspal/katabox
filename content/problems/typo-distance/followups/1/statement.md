The spell checker now only needs a yes or no. A fix is still one of: **insert** a letter
anywhere, **delete** a letter, or **replace** a letter with another. Return `true` if
`typed` can be turned into `meant` with **at most `k` fixes**, and `false` otherwise.

The words are much longer than before (up to 2·10⁴ letters), but `k` is small.

### Example 1

```
typed = "kitten", meant = "sitting", k = 3
Output: true
```

Replace `k` with `s`, replace `e` with `i`, and insert `g` at the end: exactly 3 fixes.

### Example 2

```
typed = "eacadd", meant = "edcadd", k = 2
Output: true
```

### Example 3

```
typed = "aacbc", meant = "aacbbb", k = 2
Output: true
```

Replace the last `c` with `b` and insert one more `b`.

### Constraints

- 1 ≤ typed.length, meant.length ≤ 2·10⁴
- typed and meant contain only lowercase English letters.
- 0 ≤ k ≤ 20
