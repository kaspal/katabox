## La idea

Un caminante empieza en la celda superior izquierda de una cuadrícula y debe llegar a la celda inferior derecha, moviéndose solo **a la derecha** o **hacia abajo**. Algunas celdas están **bloqueadas**. ¿Cuántos caminos distintos hay? Enumerarlos todos es inútil: una cuadrícula abierta de 10 por 10 ya tiene 48 620.

Todo camino que entra en una celda llega **desde arriba** o **desde la izquierda**, nunca de las dos formas, así que las cuentas simplemente se suman. `ways[r][c]` es el número de caminos desde la esquina superior izquierda hasta la celda `(r, c)`:

- Una celda **bloqueada** recibe 0: ningún camino puede pisarla.
- La celda **superior izquierda** recibe 1 (el camino que todavía no se ha movido), salvo que esté bloqueada.
- Cualquier otra celda recibe `arriba + izquierda`, donde un vecino fuera de la cuadrícula cuenta como 0.

El ejemplo rellena la tabla fila a fila, de izquierda a derecha, así que los dos vecinos están listos cuando una celda los necesita. La salida es `[ways[R - 1][C - 1]]`, la cuenta de la esquina inferior derecha. El mapa se escribe fila a fila, con las filas separadas por `/`, `.` para una celda abierta y `#` para una bloqueada.

### La entrada por defecto

Cinco filas de seis celdas: `....../.#..#./....../..#.../......`. La fila 0 es toda unos: una sola forma por el borde de arriba. La fila 1 es `1, 0, 1, 2, 0, 1`: las celdas bloqueadas valen 0, y la celda que sigue al primer bloqueo solo recibe su 1 desde arriba. La fila 2 es `1, 1, 2, 4, 4, 5`. La fila 3 es `1, 2, 0, 4, 8, 13`. La fila 4 es `1, 3, 3, 7, 15, 28`.

La salida es `[28]`. Sin los tres bloqueos, la misma cuadrícula tendría 126 caminos.

## Pasos

1. Crea una tabla de `R` filas y `C` columnas.
2. Para cada fila `r` y columna `c`, en orden: si la celda está bloqueada, escribe 0; si no, si es la superior izquierda, escribe 1; si no, escribe `arriba + izquierda`, con 0 para un vecino que no existe.
3. Devuelve la celda inferior derecha.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(R · C)`: cada celda suma dos números |
| Espacio extra | `O(R · C)` para la tabla |

## Errores comunes

- **Olvidar que un bloqueo deja aislado lo que hay detrás.** Una celda cuyos vecinos de arriba y de la izquierda valen 0 recibe 0 aunque esté abierta; el ajuste "amurallado" deja un único camino.
- **Un inicio o un final bloqueados.** Entonces la respuesta es 0; el ejemplo escribe 0 en la esquina superior izquierda antes de comprobar si es el inicio.
- **Cuentas enormes.** Los caminos crecen muy deprisa con el tamaño de la cuadrícula; los problemas reales piden la cuenta módulo algún número, aplicado tras cada suma.

## Cuándo usarlo

Úsalo para contar rutas con movimientos restringidos y, con `max` en lugar de la suma, para encontrar la mejor ruta. Los dos problemas enlazados usan esta tabla: Paseos con cortes cuenta paseos que esquivan cruces cerrados, y Camino del huerto recoge el mayor número de manzanas por el camino.

## Temas relacionados

- [Subir escaleras](learn:dynamic-programming/climbing-stairs): la misma cuenta, a lo largo de una línea.
- [Distancia de edición](learn:dynamic-programming/edit-distance): una tabla que lee sus vecinos de arriba y de la izquierda, y la diagonal.
- [Búsqueda en anchura en una cuadrícula](learn:grid-pathfinding/grid-bfs): la ruta más corta cuando se puede ir en cualquier dirección.
- [Subsecuencia común más larga](learn:dynamic-programming/lcs): otra cuadrícula que se rellena fila a fila.
