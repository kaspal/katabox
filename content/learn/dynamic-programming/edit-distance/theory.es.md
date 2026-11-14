## La idea

¿Cuántas correcciones de una letra convierten la palabra `a` en la palabra `b`? Una corrección es una de tres operaciones, cada una con coste 1: **insertar** una letra, **borrar** una letra o **sustituir** una letra por otra. El menor número de correcciones es la **distancia de edición** (también llamada distancia de Levenshtein). Los correctores ortográficos ordenan sus sugerencias por ella.

`cost[i][j]` es el menor número de correcciones que convierten las primeras `i` letras de `a` en las primeras `j` letras de `b`. Los bordes son fáciles: convertir `i` letras en nada cuesta `i` borrados, así que la columna 0 vale `i`; construir `j` letras desde nada cuesta `j` inserciones, así que la fila 0 vale `j`.

En cada una de las demás celdas se miran las últimas letras, `a[i - 1]` y `b[j - 1]`. Si son **iguales**, no hace falta corregirlas: se copia la **diagonal**, `cost[i - 1][j - 1]`. Si son distintas, la última corrección es una de tres, y cada vecino representa una:

- **Diagonal**, `cost[i - 1][j - 1]`: **sustituir** `a[i - 1]` por `b[j - 1]`, después de convertir entre sí los prefijos más cortos.
- **Arriba**, `cost[i - 1][j]`: **borrar** `a[i - 1]`; el resto de `a` ya se convirtió en las `j` letras.
- **Izquierda**, `cost[i][j - 1]`: **insertar** `b[j - 1]` al final; las `i` letras ya se convirtieron en las primeras `j - 1`.

La celda toma la menor de las tres, más 1. La salida es la esquina, `[cost[m][n]]`.

### La entrada por defecto

Palabras `carpet` y `carrot`. La fila 0 va de 0 a 6, y la columna 0 también. Las tres primeras letras coinciden, así que la diagonal se queda en 0 hasta `cost[3][3]`. En `cost[4][4]`, la `p` se encuentra con la `r`: la diagonal ofrece 0, así que la celda vale 1, una sustitución. En `cost[5][5]`, la `e` se encuentra con la `o`: la diagonal ofrece 1 y la celda vale 2. Las últimas letras son las dos `t`, así que la esquina copia la diagonal: 2.

La salida es `[2]`: sustituir la `p` por `r` y la `e` por `o`.

## Pasos

1. Crea una tabla de `m + 1` filas y `n + 1` columnas; pon `cost[i][0] = i` y `cost[0][j] = j`.
2. Rellénala fila a fila: si `a[i - 1] = b[j - 1]`, copia la diagonal; si no, toma 1 + la menor entre la diagonal, arriba e izquierda.
3. Devuelve `cost[m][n]`.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(m · n)`: una celda por cada par de prefijos |
| Espacio extra | `O(m · n)` para la tabla |

## Errores comunes

- **Olvidar los bordes.** Con ceros en la fila 0 y la columna 0, insertar o borrar al principio saldría gratis.
- **Sumar 1 cuando coinciden.** Las letras iguales copian la diagonal; el 1 es solo para una corrección real.
- **Letras intercambiadas.** Dos letras vecinas en el orden equivocado cuestan aquí dos sustituciones, como en el ajuste "letras cambiadas" (`form` y `from`). Contar un intercambio como una sola corrección es otra distancia.

## Cuándo usarlo

Úsalo para medir lo lejos que están dos cadenas: sugerencias de erratas, búsqueda aproximada, comparar secuencias cortas. El problema enlazado, Distancia de erratas, es justo esto.

## Temas relacionados

- [Subsecuencia común más larga](learn:dynamic-programming/lcs): la misma cuadrícula de prefijos, conservando letras en lugar de corregirlas.
- [Caminos por una cuadrícula](learn:dynamic-programming/grid-paths): una tabla que se rellena desde los vecinos de arriba y de la izquierda.
- [Mochila 0/1](learn:dynamic-programming/knapsack-01): otra tabla de dos dimensiones.
- [Hash rodante](learn:hashing/rolling-hash): coincidencias exactas rápidas, cuando no se permite ninguna corrección.
