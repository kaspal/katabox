Look at one row of the picture. Which lines of the output come from it, and how do they differ from each other?
---
Every row of the picture gives exactly `K` output lines, and those `K` lines are identical. Inside each of them, every character of the row appears `K` times in a row.
---
For each picture row, build the wide row once by repeating each character `K` times (`#.#` with `K = 2` gives `##..##`), then add that wide row to the output `K` times. Print all the lines at the end in one write.
