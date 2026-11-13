## Enfoque: sumas prefijas en 2-D

Construye una tabla `pre` con una fila y una columna extra de ceros, donde `pre[r][c]` es el total de los árboles de las filas `0..r−1` y las columnas `0..c−1`. Fila a fila, lleva el total acumulado `run` de la fila actual: `pre[r+1][c+1] = pre[r][c+1] + run`.

Una parcela `[top, left, bottom, right]` es el rectángulo de esquina grande hasta `(bottom, right)`, menos la franja de encima, menos la franja de la izquierda, más la esquina que se restó dos veces:

`pre[bottom+1][right+1] − pre[top][right+1] − pre[bottom+1][left] + pre[top][left]`.

### Paso a paso

Ejemplo 1, parcela `[1, 1, 2, 2]`: toda la cuadrícula suma 24, la fila de arriba 5, la columna de la izquierda 10, y el árbol de la esquina (3) se quitó dos veces: 24 − 5 − 10 + 3 = 12.

**Complejidad:** O(filas · columnas) para construirla y O(1) por consulta. Sumar cada parcela directamente es O(filas · columnas) por consulta, hasta 9·10⁹ pasos con 10⁵ parcelas grandes.
