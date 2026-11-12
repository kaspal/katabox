## Enfoque: programación dinámica sobre la cuadrícula

Todo camino hasta el árbol `(r, c)` llega desde el de arriba, `(r − 1, c)`, o desde el de la izquierda, `(r, c − 1)`. El resto del camino no importa, solo cuántas manzanas recogió. Así que el máximo de un camino que termina en `(r, c)` es

```
mejor[r][c] = grid[r][c] + max(mejor[r - 1][c], mejor[r][c - 1])
```

En la fila de arriba solo existe la vecina de la izquierda, y en la columna de la izquierda solo la de arriba. Al rellenar la tabla fila a fila, los dos valores están listos cuando hacen falta. La respuesta es `mejor[filas − 1][columnas − 1]`.

Basta con un solo arreglo de longitud `columnas`: antes de actualizar, `mejor[c]` aún guarda el valor de la fila de arriba, y `mejor[c − 1]` ya tiene el valor nuevo de la izquierda.

```
mejor = [0] * columnas
para r en filas:
    para c en columnas:
        si r == 0 y c == 0: desde = 0
        si no, si r == 0: desde = mejor[c - 1]
        si no, si c == 0: desde = mejor[c]
        si no: desde = max(mejor[c], mejor[c - 1])
        mejor[c] = grid[r][c] + desde
devolver mejor[columnas - 1]
```

### Paso a paso

Para `[[3, 1, 4], [1, 5, 9], [2, 6, 5]]` la tabla queda:

| | col 0 | col 1 | col 2 |
| --- | --- | --- | --- |
| fila 0 | 3 | 4 | 8 |
| fila 1 | 4 | 9 | 18 |
| fila 2 | 6 | 15 | 23 |

La respuesta es 23.

**Complejidad:** O(filas × columnas) de tiempo y O(columnas) de espacio adicional. Explorar cada camino con recursión es exponencial: un huerto de 20 × 20 ya tiene unos 3,5·10¹⁰ caminos.
