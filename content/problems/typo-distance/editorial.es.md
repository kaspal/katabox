## Enfoque: programación dinámica sobre prefijos

Sea `d[i][j]` el menor número de arreglos que convierten `typed[:i]` en `meant[:j]`.

- `d[i][0] = i` (borrarlo todo) y `d[0][j] = j` (insertarlo todo).
- Si las últimas letras coinciden, `typed[i−1] = meant[j−1]`, no necesitan arreglo: `d[i][j] = d[i−1][j−1]`.
- Si no, el último arreglo borra `typed[i−1]`, inserta `meant[j−1]` o sustituye una por otra:
  `d[i][j] = 1 + min(d[i−1][j], d[i][j−1], d[i−1][j−1])`.

Cada fila solo necesita la anterior, así que basta con dos filas de longitud `m + 1`.

### Paso a paso

`typed = "stake"`, `meant = "skate"` (filas: prefijos de `typed`; columnas: prefijos de `meant`):

| | "" | s | k | a | t | e |
| --- | --- | --- | --- | --- | --- | --- |
| "" | 0 | 1 | 2 | 3 | 4 | 5 |
| s | 1 | 0 | 1 | 2 | 3 | 4 |
| t | 2 | 1 | 1 | 2 | 2 | 3 |
| a | 3 | 2 | 2 | 1 | 2 | 3 |
| k | 4 | 3 | 2 | 2 | 2 | 3 |
| e | 5 | 4 | 3 | 3 | 3 | **2** |

**Complejidad:** O(n·m) en tiempo y O(m) en espacio: 10⁶ casillas para dos palabras de 1000 letras. La recursión simple se ramifica en tres en cada letra distinta y es exponencial.
