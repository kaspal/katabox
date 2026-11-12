Hay demasiados caminos para probarlos todos: unos C(598, 299) en un huerto de 300 × 300. Pero la mejor forma de llegar a un árbol solo depende de las mejores formas de llegar al de justo encima y al de justo a su izquierda.
---
Sea `mejor[r][c]` el máximo de manzanas de un camino que termina en `(r, c)`. Solo se puede llegar desde `(r − 1, c)` o desde `(r, c − 1)`.
---
`mejor[r][c] = grid[r][c] + max(mejor[r − 1][c], mejor[r][c − 1])`, usando solo la vecina que exista en la fila de arriba y en la columna de la izquierda. Rellena la tabla fila a fila; la respuesta es la casilla de abajo a la derecha. Basta con una fila de memoria.
