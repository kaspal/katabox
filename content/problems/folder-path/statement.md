An old archive server takes folder paths typed by hand, and they are often messy. A path starts with `/` (the top folder) and is split by `/` into parts:

- an empty part (from `//` or a trailing `/`) and the part `.` mean "stay here";
- the part `..` means "go up to the parent folder"; at the top folder it stays at the top;
- any other part (letters, digits, `_` and dots, such as `...` or `.config`) is a folder name: go into it.

Return the clean path: `/` followed by the folder names you end up in, separated by single `/`, with no trailing `/`. The top folder itself is `/`.

### Example 1

```
path = "/photos/./2019//summer/../winter/"
Output: "/photos/2019/winter"
```

### Example 2

```
path = "/../.."
Output: "/"
```

### Example 3

```
path = "/notes/...//a"
Output: "/notes/.../a"
```

`...` is a folder name, not a move.

### Constraints

- 1 ≤ path.length ≤ 3000
- path starts with `/` and has only letters, digits, `.`, `_` and `/`.
