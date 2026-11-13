## Enfoque 1: búsqueda binaria en cada fila

Cada fila está ordenada, así que una búsqueda binaria por fila encuentra la entrada en O(filas · log columnas). Las filas cuyo primer asiento es mayor que la entrada se pueden saltar, y también aquellas cuyo último asiento es menor.

## Enfoque 2: una escalera desde la esquina superior derecha

Colócate en el asiento de arriba a la derecha `(r, c) = (0, columnas − 1)` y compara:

- `seats[r][c] = ticket`: encontrada;
- `seats[r][c] > ticket`: cada asiento debajo de él en la columna `c` es aún mayor, así que la columna `c` no puede tener la entrada: `c--`;
- `seats[r][c] < ticket`: cada asiento a su izquierda en la fila `r` es aún menor, así que la fila `r` queda fuera: `r++`.

Cada paso descarta una fila o una columna, así que tras filas + columnas pasos como mucho, o encuentras la entrada o te sales del plano (no está).

### Paso a paso

Ejemplo 1, entrada 11: empieza en 9 (< 11, baja), 12 (> 11, izquierda), 8 (< 11, baja), 11: encontrada.

**Complejidad:** O(filas · log columnas) u O(filas + columnas), con espacio O(1).
