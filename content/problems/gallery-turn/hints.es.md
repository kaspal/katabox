Dibuja una cuadrícula de 2×3 y su versión girada. ¿Adónde va la baldosa de la fila `i`, columna `j`?
---
La cuadrícula girada tiene `c` filas. Su fila `j` es la columna `j` original, leída desde la última fila hacia arriba.
---
Crea `out` con `c` filas de `r` valores y pon `out[j][r - 1 - i] = grid[i][j]` para cada baldosa.
