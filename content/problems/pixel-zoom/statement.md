A small pixel-art editor draws pictures with characters: every character is one pixel. Its zoom tool enlarges a picture by a whole factor `K`, turning every pixel into a block of `K × K` copies of the same character, so the picture keeps its shape and becomes `K` times wider and `K` times taller.

This is a **program problem**: read the picture and the zoom factor from standard input and print the zoomed picture to standard output.

## Input format

- The first line holds `R C K`: the number of rows, the number of columns and the zoom factor.
- Then `R` lines, the picture's rows from top to bottom, each with exactly `C` characters. Every character is one of `.`, `#`, `*`, `o` and `+`.

## Output format

Print the zoomed picture: `R · K` lines of `C · K` characters each.

### Example 1

Input:

```
2 3 2
#.#
.o.
```

Output:

```
##..##
##..##
..oo..
..oo..
```

Each pixel becomes a 2 × 2 block: the first row `#.#` turns into two copies of `##..##`.

### Example 2

Input:

```
1 1 3
*
```

Output:

```
***
***
***
```

### Example 3

Input:

```
3 2 1
+.
.+
##
```

Output:

```
+.
.+
##
```

With `K = 1` the picture stays as it is.

### Constraints

- 1 ≤ R ≤ 40
- 1 ≤ C ≤ 40
- 1 ≤ K ≤ 8
- Every row has exactly `C` characters from `.`, `#`, `*`, `o`, `+`.
