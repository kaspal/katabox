La tabla completa del problema base tendría 5·10⁴ · 5·10⁴ = 2,5·10⁹ celdas: demasiado lenta
y demasiado grande. Pero solo necesitas saber si la distancia es como mucho `k`. ¿Qué
celdas pueden importar todavía?
---
La celda `(i, j)` compara las primeras `i` letras de `typed` con las primeras `j` de
`meant`. Sus longitudes se diferencian en `|i − j|`, así que esa celda vale al menos
`|i − j|`. Si las palabras completas se diferencian en longitud en más de `k`, la
respuesta ya es `false`.
---
Rellena solo las celdas con `|i − j| ≤ k`, una banda de anchura `2k + 1` alrededor de la
diagonal, tratando cada celda de fuera como `k + 1` (y limitando los valores a `k + 1`).
Después comprueba si la última celda es como mucho `k`.
