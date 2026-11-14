## La idea

Dos palabras comparten una **subsecuencia común** cuando las mismas letras aparecen en ambas, en el mismo orden, con cualquier cosa entre medias. ¿Cuál es la subsecuencia compartida más larga? Enumerar todas las subsecuencias de una palabra y comprobarlas en la otra duplica el trabajo con cada letra. La tabla, en cambio, compara **prefijos**.

`best[i][j]` es la longitud de la subsecuencia común más larga entre las primeras `i` letras de la palabra `a` y las primeras `j` letras de la palabra `b`. La fila 0 y la columna 0 representan un **prefijo vacío**, así que son todo ceros. En cada una de las demás celdas se miran las últimas letras, `a[i - 1]` y `b[j - 1]`:

- Si **coinciden**, esa letra puede cerrar una subsecuencia compartida: `best[i - 1][j - 1] + 1`, la celda **diagonal** más uno.
- Si no, una de las dos letras no se usa. Se quita la última letra de `a` (la celda **de arriba**) o la de `b` (la celda **de la izquierda**), y se toma la mayor.

La respuesta está en la celda inferior derecha. Una **reconstrucción** desde ahí recupera las letras: si coinciden, se apunta la letra y se avanza en diagonal; si no, se avanza hacia el vecino mayor, **hacia arriba** en caso de empate. Las letras salen al revés, así que el ejemplo les da la vuelta. La salida es la longitud y después las letras como números del 0 al 25 (a = 0), que la aplicación muestra como letras. Las palabras se escriben en dos campos, con letras de la a a la z.

### La entrada por defecto

Palabras `planet` y `anklet`. La fila de la `p` es toda ceros: no hay coincidencias. La fila de la `l` llega a 1 en la `l` de `anklet`. La fila de la `a` vale 1 desde su primera columna. La fila de la `n` coincide en la columna 2 con `1 + 1 = 2`. La fila de la `e` coincide en la columna 5 con `2 + 1 = 3`. La fila de la `t` coincide en la esquina con `3 + 1 = 4`.

La reconstrucción toma la `t` y la `e` en diagonal, avanza dos veces a la izquierda dejando atrás la `l` y la `k` (la celda de la izquierda vale 2 y la de arriba 1), y después toma la `n` y la `a`. La salida es `[4, 0, 13, 4, 19]`: las letras `ante`.

## Pasos

1. Crea una tabla de `m + 1` filas y `n + 1` columnas, toda a ceros.
2. Rellénala fila a fila: si coinciden, la diagonal + 1; si no, la mayor entre arriba e izquierda.
3. Reconstruye desde `(m, n)`: si coinciden, apunta la letra y ve en diagonal; si no, sube si `arriba ≥ izquierda` y, si no, ve a la izquierda.
4. Devuelve la longitud y las letras apuntadas, en orden inverso.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(m · n)`: una celda por cada par de prefijos |
| Espacio extra | `O(m · n)` para la tabla, que la reconstrucción necesita |

## Errores comunes

- **Desfase de uno.** La celda `(i, j)` compara `a[i - 1]` con `b[j - 1]`; la fila y la columna extra son las que hacen funcionar el prefijo vacío.
- **Una subcadena no es una subsecuencia.** Una **subcadena** común tiene que ser contigua y necesita otra regla: si no coinciden, la celda vuelve a 0.
- **Más de una respuesta.** Varias subsecuencias pueden tener la misma longitud máxima; la regla de empate decide cuál encuentra la reconstrucción.

## Cuándo usarlo

Úsalo para comparar dos secuencias: diferencias entre archivos, melodías compartidas, fragmentos de ADN. El problema enlazado, La melodía compartida, es esta tabla sobre listas de notas.

## Temas relacionados

- [Distancia de edición](learn:dynamic-programming/edit-distance): la misma cuadrícula de prefijos, contando correcciones en su lugar.
- [Subsecuencia creciente más larga](learn:dynamic-programming/lis): una sola secuencia, valores que crecen.
- [Mochila 0/1](learn:dynamic-programming/knapsack-01): otra tabla de dos dimensiones con reconstrucción.
- [Fibonacci: memo y tabla](learn:dynamic-programming/fibonacci-memo-table): donde empiezan las tablas de subproblemas.
