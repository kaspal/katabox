## Approach: widen each row, then repeat it

Zooming by `K` stretches the picture in two independent directions:

- **sideways**, every character becomes `K` copies of itself: `#.#` with `K = 2` becomes `##..##`;
- **downwards**, every widened row is printed `K` times.

So for each of the `R` picture rows, build its wide row once and add it to the output `K` times:

```
for row in picture:
    wide = row with every character repeated K times
    add wide to the output K times
```

**Why it is correct:** in the zoomed picture, the character at line `y`, column `x` (from 0) belongs to the block of the pixel at row `y / K`, column `x / K` (whole division), and must be a copy of it. In the wide row built from row `r`, column `x` holds character `x / K` of that row; and lines `r · K` to `r · K + K − 1` are exactly the `K` copies of that wide row, which are the lines with `y / K = r`. Every output character therefore matches its pixel.

**Complexity:** O(R · C · K²) time, which is the size of the output (at most 320 lines of 320 characters, about 10⁵ characters). Nothing faster is possible, since every character must be printed.

**Reading and writing:** read all input at once. The rows contain no spaces, so you can split the whole input on whitespace (three numbers, then `R` rows) or split it into lines; with lines, strip a trailing `\r` in case the input has Windows line ends. Collect the output lines and print them once. Repeating a string: `ch * k` (Python), `ch.repeat(k)` (TypeScript), `str_repeat($ch, $k)` (PHP), `.repeat(k)` on a `&str` (Rust).
