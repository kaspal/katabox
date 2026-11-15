## La idea

`GROUP BY` condensa cada grupo en una fila. Una **función de ventana** también calcula sobre un grupo, pero **todas las filas se quedan**: cada fila recibe un valor extra calculado a partir de las filas que la rodean, su **ventana**.

`SUM(v) OVER (PARTITION BY p ORDER BY id)` se lee así:

- `PARTITION BY p` divide las filas en particiones, una por cada valor de `p`. Las filas nunca ven otras particiones.
- `ORDER BY id` ordena las filas dentro de cada partición.
- `SUM(v)` suma `v` sobre la ventana: por defecto, desde la primera fila de la partición hasta la fila actual. El resultado es un **total acumulado**.

Si las filas ya llegan en orden de `id`, basta una pasada. Guarda un total acumulado por partición en una tabla hash. Para cada fila, suma su `v` al total de su partición (o empieza el total en `v` si la partición es nueva) y escribe ese total junto a la fila. Las filas de distintas particiones pueden intercalarse libremente; cada una toca solo su propio total.

### La entrada por defecto

El arreglo `[1, 50, 2, 20, 1, 15, 1, 5, 2, 40]` contiene pares `(p, v)`, con ids del 1 al 5 en orden: `(1, 50)`, `(2, 20)`, `(1, 15)`, `(1, 5)`, `(2, 40)`. Piensa en dos cuentas, 1 y 2, y sus pagos.

La fila 1 empieza la partición 1 en 50. La fila 2 empieza la partición 2 en 20. La fila 3 continúa la partición 1: 50 + 15 = 65. La fila 4: 65 + 5 = 70. La fila 5 continúa la partición 2: 20 + 40 = 60.

La salida es `[50, 20, 65, 70, 60]`: las cinco filas se conservan, cada una con el total de su partición hasta ese momento, en 2 particiones.

## Pasos

1. Lee la entrada como pares `(p, v)`, ya en orden de `id`.
2. Para cada fila, si `p` todavía no tiene total, ponlo en `v`; si no, súmale `v`.
3. Escribe el total actual de la partición como valor de esta fila.
4. Devuelve los totales en el orden de las filas.

## Coste

Con `n` filas y `p` particiones:

| Caso | Coste |
|---|---|
| Mejor y promedio | `O(n)`: filas ya ordenadas, una pasada |
| Peor | `O(n log n)`: primero hay que ordenar las filas por partición y orden |
| Espacio extra | `O(p)`: un total acumulado por partición |

## Errores comunes

- **El marco por defecto.** Con `ORDER BY` dentro de `OVER`, la ventana por defecto es `RANGE BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW`, que incluye todas las filas empatadas con la actual en el valor de `ORDER BY`. Aquí `id` es único, así que equivale a `ROWS UNBOUNDED PRECEDING`, el marco que escribe la comprobación del ejemplo. Con valores repetidos, escribe `ROWS` para obtener un verdadero total acumulado fila a fila.
- **Omitir `ORDER BY`.** Sin él, la ventana es toda la partición, y cada fila recibe el total completo de la partición, no uno acumulado.
- **Esperar menos filas.** Una función de ventana nunca elimina filas; para obtener una fila por partición, usa `GROUP BY`.
- **Filtrar por el resultado en `WHERE`.** Las funciones de ventana se calculan después de `WHERE`, así que filtra por ellas en una consulta externa.

## Cuándo usarlo

Usa una función de ventana cuando cada fila necesite contexto de sus vecinas: saldos acumulados, clasificaciones, el cambio desde la fila anterior. Las demás funciones de ventana siguen el mismo patrón `OVER (...)`: `ROW_NUMBER()` numera las filas de cada partición, `RANK()` hace lo mismo pero da el mismo puesto a los empates, y `LAG(v)` lee `v` de la fila anterior. El problema enlazado, Registro de kilómetros, responde totales por rangos con sumas acumuladas, la misma idea de las sumas prefijas.

## Temas relacionados

- Sumas prefijas: un total acumulado sobre un arreglo, el corazón de este tema.
- GROUP BY: una fila por grupo en lugar de una por fila.
- Tabla hash: encadenamiento: la tabla que guarda los totales por partición.
- Árbol de Fenwick: totales acumulados que siguen siendo correctos cuando cambian los valores.
