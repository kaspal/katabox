## La idea

Una **unión** (join) pega dos tablas: `SELECT L.key, L.a, R.b FROM L JOIN R ON L.key = R.key` empareja cada fila de `L` con cada fila de `R` que tenga la misma `key`, y produce una fila por pareja. Las filas sin pareja en el otro lado simplemente desaparecen (es una unión interna, inner join).

La forma más sencilla es la **unión por bucles anidados** (nested-loop join): para cada fila de `L`, recorre toda `R` y quédate con las parejas cuyas claves son iguales. Siempre es correcta y no necesita memoria extra, pero compara todas las parejas, `|L| · |R|` comparaciones, aunque casi nada coincida.

La **unión por hash** (hash join), la que muestra la animación, trabaja en dos fases:

- **Construcción**: lee `R` una vez y mete cada fila en un índice hash bajo su `key`. Las filas con la misma clave comparten una entrada, como una lista de posiciones de fila.
- **Sondeo**: lee `L` una vez. Para cada fila, busca su `key` en el índice: una búsqueda hash en lugar de recorrer `R`. Cada fila de R que aparece allí es una coincidencia, y cada coincidencia se convierte en una fila de salida.

La salida mantiene el orden de `L` y, dentro de una fila de `L`, el orden de `R`.

### La entrada por defecto

El arreglo `[3, 1, 10, 2, 20, 3, 30, 2, 5, 3, 7, 3, 8, 4, 9]` empieza por `3`, el número de filas de `L`. Los tres pares siguientes son `L` como `(key, a)`: `(1, 10)`, `(2, 20)`, `(3, 30)`. Los pares restantes son `R` como `(key, b)`: `(2, 5)`, `(3, 7)`, `(3, 8)`, `(4, 9)`.

Construcción: el índice recibe la clave 2 con la fila #1 de R, la clave 3 con las filas #2 y #3, y la clave 4 con la fila #4. Sondeo: la clave 1 no encuentra nada, así que `(1, 10)` se descarta. La clave 2 encuentra una fila y produce `(2, 20, 5)`. La clave 3 encuentra dos filas y produce `(3, 30, 7)` y `(3, 30, 8)`. Nadie pregunta por la fila `(4, 9)` de R.

La salida es `[2, 20, 5, 3, 30, 7, 3, 30, 8]`: tres filas unidas con 4 inserciones y 3 sondeos, cuando una unión por bucles anidados habría comparado las 12 parejas.

## Pasos

1. Lee el número de filas de `L` y divide los pares en `L` (esa cantidad) y `R` (el resto).
2. Construcción: para cada fila de `R`, añade su `b` a la entrada del índice de su `key`.
3. Sondeo: para cada fila de `L`, busca su `key`; por cada valor encontrado, produce `(key, a, b)`.
4. Devuelve las filas de salida en ese orden.

## Coste

Con `m` filas en `L` y `n` filas en `R`:

| Caso | Coste |
|---|---|
| Mejor | `O(m + n)`: pocas coincidencias o ninguna |
| Promedio | `O(m + n + salida)`: una inserción por fila de `R`, un sondeo por fila de `L`, una escritura por fila unida |
| Peor | `O(m · n)`: todas las filas comparten una clave, así que la propia salida tiene `m · n` filas |
| Espacio extra | `O(n)` para el índice |

Una unión por bucles anidados cuesta `O(m · n)` en todos los casos.

Las bases de datos reales usan también un tercer método, la **unión por ordenación y mezcla** (sort-merge join): ordenar ambas tablas por `key` y recorrerlas a la par, como el paso de mezcla de la ordenación por mezcla. Brilla cuando las dos entradas ya llegan ordenadas, por ejemplo desde un índice. El planificador de consultas elige entre los tres según el tamaño de las tablas, los índices disponibles y la memoria; los bucles anidados suelen ser la mejor opción cuando un lado es diminuto o `R.key` tiene un índice.

## Errores comunes

- **Construir sobre la tabla más grande.** El índice tiene que caber en memoria; las bases de datos construyen sobre el lado pequeño y sondean con el grande.
- **Las claves repetidas multiplican filas.** Dos filas de `L` y tres de `R` con la misma clave dan seis filas de salida. Una unión puede ser mucho mayor que cualquiera de las dos tablas.
- **Claves NULL.** En SQL, `NULL = NULL` no es verdadero, así que las filas con clave `NULL` nunca se unen.
- **Olvidar la condición `ON`.** Sin ella, todas las parejas coinciden: un producto cruzado de `|L| · |R|` filas.

## Cuándo usarlo

Usa una unión por hash siempre que tengas que emparejar registros de dos listas por una clave igual y ninguna de las dos esté ordenada: asociar pedidos con clientes, combinar dos exportaciones por id. El problema enlazado, Suma de dos, usa la misma idea de construir y sondear: guardar valores en una tabla hash y luego buscar cada pareja en tiempo constante.

## Temas relacionados

- Tabla hash: encadenamiento: el índice que llena la fase de construcción.
- Tabla hash: direccionamiento abierto: otra forma de guardar ese índice.
- Ordenación por mezcla (Merge Sort): el paso de mezcla detrás de la unión por ordenación y mezcla.
- GROUP BY: la misma tabla hash, usada para condensar filas en lugar de emparejarlas.
