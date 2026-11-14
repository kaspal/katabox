## La idea

Una **subsecuencia** conserva algunos de los valores en su orden original y se salta el resto. La tarea es encontrar la más larga que crece de forma estricta: cada valor mayor que el anterior. Probar todos los subconjuntos de posiciones duplica el trabajo con cada valor, pero toda respuesta tiene que **terminar en algún sitio**, y esa es la pregunta más pequeña que hace la tabla.

`length[i]` es la longitud de la subsecuencia creciente más larga que **termina en el índice `i`**. Por sí solo, un valor es una subsecuencia de longitud 1, así que todas las celdas empiezan en 1. Después el ejemplo mira cada índice anterior `j`: si `values[j] < values[i]`, el valor de `i` puede prolongar lo que terminaba en `j`, lo que da `length[j] + 1`. La celda se queda con la mayor oferta, y `prev[i]` recuerda qué `j` la dio. Solo una oferta **estrictamente** mayor sustituye a la actual, así que en un empate se queda el `j` más temprano.

La respuesta termina en el índice con la mayor longitud; si hay empate, el ejemplo toma el **primero** de esos índices. Siguiendo `prev` desde ahí se recorre la subsecuencia hacia atrás. La salida es la longitud y después la subsecuencia en orden.

### La entrada por defecto

Valores `[7, 3, 8, 4, 9, 2, 10, 6, 11]`. Las longitudes salen `[1, 1, 2, 2, 3, 1, 4, 3, 5]`. El índice 2 (valor 8) prolonga el 7, y el 3 ofrece el mismo 2, que no lo sustituye. El índice 4 (valor 9) recibe 3 desde el 8. El índice 6 (valor 10) recibe 4 desde el 9. El índice 7 (valor 6) recibe 3 pasando por el 3 y el 4. El índice 8 (valor 11) recibe 5 desde el 10.

La reconstrucción empieza en el índice 8 y sigue `prev`: 11, 10, 9, 8, 7. La salida es `[5, 7, 8, 9, 10, 11]`. La cadena 3, 8, 9, 10, 11 es igual de larga; la regla de empate eligió el 7 porque venía antes.

## Pasos

1. Pon cada `length[i]` a 1 y cada `prev[i]` a -1.
2. Para cada `i`, y cada `j < i` con `values[j] < values[i]`: si `length[j] + 1 > length[i]`, pon `length[i]` a ese valor y `prev[i] = j`.
3. Sea `end` el primer índice con la mayor longitud.
4. Sigue `prev` desde `end` hasta llegar a -1, recogiendo valores; dales la vuelta y pon la longitud delante.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(n²)`: cada índice mira todos los índices anteriores |
| Espacio extra | `O(n)` para `length` y `prev` |

## Errores comunes

- **Tomar la última celda como respuesta.** La subsecuencia más larga no tiene por qué terminar en el último valor; busca el máximo.
- **Estricto o no estricto.** `<` hace que la subsecuencia crezca de forma estricta; `≤` dejaría repetir valores iguales. Lee cuál de los dos pide la tarea.
- **Entradas grandes.** Con decenas de miles de valores, `n²` es demasiado lento. El método de la **paciencia** guarda, para cada longitud, el menor valor que puede terminarla, y encuentra el sitio de cada valor nuevo con una búsqueda binaria: `O(n log n)`.

## Cuándo usarlo

Úsalo cuando hay que elegir elementos en orden y cada uno debe superar al anterior: precios que suben, cajas que encajan unas en otras tras ordenarlas, la cadena más larga de cualquier cosa. El problema enlazado, Torres crecientes, pide esta longitud, pero con hasta 50 000 torres necesita el método de la paciencia.

## Temas relacionados

- [Subarreglo de suma máxima (Kadane)](learn:dynamic-programming/maximum-subarray): la mejor respuesta que termina en cada índice, mirando solo un paso atrás.
- [El ladrón de casas](learn:dynamic-programming/house-robber): tomar o saltar, un índice cada vez.
- [Subsecuencia común más larga](learn:dynamic-programming/lcs): subsecuencias que comparten dos secuencias.
- [Búsqueda binaria](learn:searching/binary-search): el paso que hace rápido el método de la paciencia.
