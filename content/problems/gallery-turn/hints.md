Draw a 2×3 grid and its turned version. Where does the tile at row `i`, column `j` end up?
---
The turned grid has `c` rows. Its row `j` is the original column `j`, read from the bottom row up.
---
Build `out` with `c` rows of `r` values and set `out[j][r - 1 - i] = grid[i][j]` for every tile.
