## La idea

`SELECT g, COUNT(*), SUM(v) FROM t GROUP BY g ORDER BY g` condensa una tabla: todas las filas con el mismo `g` se convierten en una sola fila de salida, que lleva cuántas filas tenía el grupo y la suma de sus `v`. Entran muchas filas; sale una fila por cada `g` distinto.

La animación muestra la **agregación por hash**, la forma en que lo hacen la mayoría de las bases de datos cuando los grupos caben en memoria. Una tabla hash asocia cada clave de grupo con un pequeño estado acumulado, aquí el par `(count, sum)`. Basta una pasada por las filas:

- Si el `g` de la fila es **nuevo**, abre un grupo con cuenta 1 y suma `v`.
- Si el grupo **ya existe**, suma 1 a su cuenta y `v` a su suma.

Nunca se compara una fila con otras; cada fila toca solo su propio grupo. Al terminar la pasada, la tabla tiene la cuenta y la suma finales de cada grupo. Una tabla hash no guarda ningún orden útil, así que `ORDER BY g` es un paso aparte: ordenar las claves de grupo y producir los grupos en ese orden.

La alternativa es la **agrupación por ordenación**: ordenar las filas por `g`, para que las claves iguales queden juntas, y recorrerlas una vez cerrando un grupo cada vez que cambia la clave. Cuesta una ordenación completa, `O(n log n)`, pero no necesita tabla hash y entrega los grupos ya ordenados. Las bases de datos la eligen cuando la entrada ya viene ordenada, por ejemplo leída a través de un índice sobre `g`.

### La entrada por defecto

El arreglo `[1, 10, 2, 5, 1, 30, 2, 7, 1, 2]` contiene pares `(g, v)`: cinco filas, `(1, 10)`, `(2, 5)`, `(1, 30)`, `(2, 7)`, `(1, 2)`.

La fila 1 abre el grupo 1: cuenta 1, suma 10. La fila 2 abre el grupo 2: cuenta 1, suma 5. La fila 3 suma 30 al grupo 1: cuenta 2, suma 40. La fila 4 suma 7 al grupo 2: cuenta 2, suma 12. La fila 5 suma 2 al grupo 1: cuenta 3, suma 42. Después los grupos salen por `g`: primero el grupo 1 y luego el grupo 2.

La salida es `[1, 3, 42, 2, 2, 12]`: el grupo 1 tiene 3 filas que suman 42 y el grupo 2 tiene 2 filas que suman 12.

## Pasos

1. Lee la entrada como pares `(g, v)`.
2. Para cada fila, busca `g` en la tabla hash; si no está, empiézalo en `(0, 0)`, y luego suma 1 a la cuenta y `v` a la suma.
3. Ordena las claves de grupo.
4. Para cada clave, en orden, produce `g`, su cuenta y su suma.

## Coste

Con `n` filas y `g` grupos:

| Caso | Coste |
|---|---|
| Mejor | `O(n)`: un solo grupo, nada que ordenar |
| Promedio y peor | `O(n + g log g)`: una pasada y después ordenar las claves de grupo |
| Espacio extra | `O(g)`: una entrada por grupo |

## Errores comunes

- **¿WHERE o HAVING?** `WHERE` filtra filas **antes** de agrupar, así que no puede ver `COUNT(*)` ni `SUM(v)`. `HAVING` filtra grupos **después** de agrupar: `HAVING COUNT(*) > 2` conserva solo los grupos grandes.
- **Esperar los grupos ordenados.** Sin `ORDER BY`, SQL no promete ningún orden; una tabla hash produce los grupos en el orden que le parezca.
- **Seleccionar una columna que no está agrupada.** En `SELECT g, v ... GROUP BY g`, un grupo tiene muchos valores de `v`; la mayoría de las bases de datos lo rechazan, y las que lo aceptan devuelven uno cualquiera.
- **`COUNT(*)` frente a `COUNT(v)`.** El primero cuenta filas; el segundo se salta las filas donde `v` es `NULL`.

## Cuándo usarlo

Usa la agregación por hash siempre que tengas que sumar, contar o promediar registros por una clave: ventas por cliente, palabras por documento, existencias por producto. En código es solo un diccionario de totales acumulados. El problema enlazado, Etiquetas de cajas, suma cantidades por producto exactamente así.

## Temas relacionados

- Tabla hash: encadenamiento: la tabla que guarda los grupos.
- Algoritmos de unión: la misma tabla hash, usada para emparejar filas en lugar de condensarlas.
- Funciones de ventana: totales por grupo que conservan todas las filas.
- Ordenación por conteo: contar por clave cuando las claves son enteros pequeños.
