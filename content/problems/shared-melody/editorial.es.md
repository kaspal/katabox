## Enfoque: subsecuencia común más larga

`L[i][j]` = la melodía compartida más larga de los prefijos `first[0..i)` y `second[0..j)`, con `L[0][·] = L[·][0] = 0`.

- Si las últimas notas coinciden, la mejor melodía puede terminar con esa nota: `L[i][j] = L[i−1][j−1] + 1`.
- Si no, al menos una de las dos últimas notas no se usa: `L[i][j] = max(L[i−1][j], L[i][j−1])`.

La respuesta es `L[n][m]`. Cada fila solo necesita la anterior, así que bastan dos arreglos de longitud m + 1.

**Complejidad:** O(n·m) en tiempo (como mucho 2,25·10⁶ celdas) y O(m) de espacio. Probar las dos opciones recursivamente sin recordar resultados es exponencial.
