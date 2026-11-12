## Enfoque: llevar cada baldosa a su nuevo sitio

Girar en el sentido del reloj lleva la fila `i`, columna `j` de una cuadrícula `r × c` a la fila `j`, columna `r − 1 − i` de una cuadrícula `c × r`. Dicho de otro modo, la fila `j` del resultado es la columna `j` original leída de abajo arriba.

Reserva el resultado de `c × r` y copia cada baldosa una vez.

### Paso a paso

`grid = [[1, 2, 3], [4, 5, 6]]` (r = 2, c = 3):

| baldosa | desde (i, j) | hasta (j, r − 1 − i) |
| --- | --- | --- |
| 1 | (0, 0) | (0, 1) |
| 4 | (1, 0) | (0, 0) |
| 3 | (0, 2) | (2, 1) |
| 6 | (1, 2) | (2, 0) |

Resultado: `[[4, 1], [5, 2], [6, 3]]`.

**Complejidad:** O(r·c) en tiempo y espacio, el tamaño de la salida. (Para cuadrículas cuadradas existe un giro sin memoria extra, pero no puede cambiar la forma de una rectangular).
