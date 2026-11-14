## La idea

Una mochila admite como mucho cierta **capacidad** de peso. Cada objeto tiene un peso y un valor y se puede meter **una vez o ninguna** (de ahí "0/1"). ¿Qué objetos dan el mayor valor total? Probar todos los subconjuntos duplica el trabajo con cada objeto, y reglas voraces como "primero el mejor valor por unidad de peso" pueden acabar en un callejón sin salida.

La tabla responde a una pregunta en dos partes: `best[i][c]` es el mayor valor usando solo los **primeros `i` objetos** con capacidad **`c`**. La fila 0 (sin objetos) es toda ceros. Para el objeto `i - 1`, con peso `w` y valor `v`, cada celda copia primero la celda **de arriba**, `best[i - 1][c]`: el objeto se queda fuera. Después, si `w ≤ c`, prueba a meterlo: `best[i - 1][c - w] + v`, lo mejor de los objetos anteriores en el hueco que queda, más este objeto. Se queda con la opción de meterlo solo si es estrictamente mejor. La respuesta está en la esquina inferior derecha.

Una **reconstrucción** recupera los objetos. Empieza en la esquina y sube fila a fila: si una celda difiere de la de arriba, su objeto se metió, así que se apunta y se avanza a la izquierda tanto como su peso. El ejemplo devuelve el mejor valor y después los objetos metidos (empezando en 0) en orden. La entrada se escribe como la capacidad seguida de pares "peso valor".

### La entrada por defecto

Capacidad 10; objetos `(5, 10)`, `(4, 7)`, `(6, 12)`, `(3, 6)` como peso y valor. La fila 1 vale 10 desde la capacidad 5. La fila 2 llega a 17 con capacidad 9: los objetos 0 y 1. La fila 3 (objeto 2, peso 6) con capacidad 10 compara el 17 de arriba con `best[2][4] + 12 = 7 + 12 = 19`, y lo mete. La fila 4 (objeto 3, peso 3) con capacidad 10 compara el 19 de arriba con `best[3][7] + 6 = 12 + 6 = 18`, y lo deja fuera.

Reconstrucción: la fila 4 es igual a la fila 3 con capacidad 10, así que el objeto 3 está fuera. La fila 3 difiere: el objeto 2 está dentro y la capacidad baja a 4. La fila 2 en 4 vale 7 frente al 0 de la fila 1: el objeto 1 está dentro, capacidad 0. La fila 1 en 0 es igual a la fila 0: el objeto 0 está fuera.

La salida es `[19, 1, 2]`. Los objetos 0, 2 y 3 dan 2 por unidad de peso, pero cualquier carga que empiece por el objeto 0 se queda como mucho en 17.

## Pasos

1. Crea una tabla de `n + 1` filas y `capacidad + 1` columnas, toda a ceros.
2. Para cada objeto `i` (peso `w`, valor `v`) y cada capacidad `c`: copia `best[i - 1][c]`; si `w ≤ c` y `best[i - 1][c - w] + v` es mayor, usa ese valor.
3. Reconstruye desde la fila `n` y la capacidad total: cada vez que `best[i][c] ≠ best[i - 1][c]`, apunta el objeto `i - 1` y resta su peso.
4. Devuelve el valor de la esquina y los objetos apuntados, en orden inverso.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(n · capacidad)`: una celda por objeto y capacidad |
| Espacio extra | `O(n · capacidad)` para la tabla, que la reconstrucción necesita |

## Errores comunes

- **Reducir la tabla a una fila recorrida de izquierda a derecha.** Con una sola fila, la celda `c - w` de la izquierda ya se ha actualizado para este objeto, así que leerla puede meter el mismo objeto dos veces. Recorriéndola de derecha a izquierda solo se leen valores del objeto anterior. (La versión de una fila además pierde la reconstrucción.)
- **Capacidades grandes.** El coste crece con el valor de la capacidad, no con su número de cifras; una capacidad de mil millones son mil millones de columnas.
- **Olvidar el peso en la reconstrucción.** Tras un objeto metido, el resto de la respuesta está en la capacidad `c - w` de la fila de arriba, no en `c`.

## Cuándo usarlo

Úsalo para presupuestos con decisiones de sí o no: hacer el equipaje, elegir proyectos con un tope de coste o escoger preguntas de examen con un límite de tiempo. El problema enlazado, El peso de la mochila, sigue este patrón.

## Temas relacionados

- [Corte de barras (mochila sin límite)](learn:dynamic-programming/rod-cutting): la misma tabla cuando un objeto se puede repetir.
- [Cambio de monedas](learn:dynamic-programming/coin-change): llegar a un total exacto con piezas reutilizables.
- [El ladrón de casas](learn:dynamic-programming/house-robber): tomar o saltar, con una tabla de una dimensión.
- [Viajante de comercio (PD con máscaras)](learn:dynamic-programming/bitmask-tsp): una tabla sobre subconjuntos de ciudades.
