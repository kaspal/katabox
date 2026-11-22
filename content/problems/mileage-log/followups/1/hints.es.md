Sumar cada celda de cada rectángulo puede costar 10⁵ celdas por pregunta, 10¹⁰ en total.
En el problema base, las sumas prefijas respondían cada viaje en O(1). ¿Funciona la misma
idea en dos dimensiones?
---
Sea `P[r][c]` el total del rectángulo que va desde la esquina superior izquierda hasta la
fila `r` y la columna `c`, sin incluirlas. Cualquier rectángulo se puede formar con cuatro
de esas esquinas.
---
Rellena `P` con `P[r+1][c+1] = miles[r][c] + P[r][c+1] + P[r+1][c] − P[r][c]`. Una
pregunta es entonces `P[v2+1][d2+1] − P[v1][d2+1] − P[v2+1][d1] + P[v1][d1]`.
