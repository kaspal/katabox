## La idea

Para contar varios patrones en un texto, lanzar una búsqueda de un solo patrón por cada uno lee el texto una y otra vez. **Aho-Corasick** lo lee una sola vez, para todos a la vez.

Empieza con un [trie](learn:trees/trie) de los patrones: cada nodo es un prefijo de algún patrón, y un nodo donde termina un patrón lo incluye en su **salida**. Bajar por el trie letra a letra sigue el texto mientras algún patrón continúe por ahí.

Cuando ningún hijo encaja con la letra siguiente, el recorrido toma un **enlace de fallo**: desde un nodo, va al sufijo propio más largo del prefijo de ese nodo que también está en el trie. Es el mismo movimiento que caer a un borde en [Knuth-Morris-Pratt](learn:strings/kmp), pero con muchos patrones. Los enlaces se construyen **en anchura**, así que el enlace de un nodo siempre está listo antes de que lo necesiten sus hijos: para un hijo al que se llega con la letra `ch`, se siguen los enlaces del padre hasta un nodo que tenga un hijo `ch`, y se enlaza ahí (o a la raíz). Un nodo también hereda la salida del nodo al que apunta su enlace, porque un patrón que termina en ese sufijo también termina aquí.

Después, el recorrido avanza letra a letra: sigue enlaces de fallo mientras el nodo actual no tenga hijo para la letra, baja si existe uno y suma uno a cada patrón de la salida del nodo. Se cuentan todas las coincidencias, también las solapadas. La salida es un contador por patrón, en el orden en que se dieron los patrones. El texto y los patrones se escriben como palabras, con letras de la a a la z. La aplicación dibuja el trie con los enlaces de fallo discontinuos.

### La entrada por defecto

Texto `abcabcab`, patrones `ab`, `bc`, `abc` y `cab`. El nodo de `abc` enlaza con `bc`, así que su salida es `abc` y `bc`; el nodo de `cab` enlaza con `ab`, así que su salida es `cab` y `ab`. El recorrido llega a `ab` y después a `abc`. Con la `a` siguiente cae de `abc` a `bc`, después a `c`, y baja a `ca` y luego a `cab`. Después, la `c` cae a `ab` y baja de nuevo a `abc`; las dos últimas letras vuelven a llegar a `cab`. La salida es `[3, 2, 2, 2]`.

## Pasos

1. Inserta cada patrón en el trie y añádelo a la salida de su último nodo.
2. Visita los nodos en anchura; enlaza cada hijo con el sufijo más largo que esté en el trie y une la salida de ese nodo a la del hijo.
3. Para cada letra del texto, sigue enlaces de fallo hasta que la letra encaje o el recorrido esté en la raíz, y baja si se puede.
4. Suma uno a cada patrón de la salida del nodo actual.

## Coste

| Caso | Coste |
|---|---|
| Todos los casos | `O(n + longitud total de los patrones + coincidencias)` |
| Espacio extra | `O(longitud total de los patrones)` para el trie y los enlaces |

Como en KMP, la profundidad crece como mucho uno por letra y cada enlace de fallo la reduce, así que las caídas suman `O(n)`.

## Errores comunes

- **Olvidar unir las salidas.** Sin eso, un patrón escondido dentro de otro más largo (aquí `bc` dentro de `abc`) se pierde.
- **Construir los enlaces en profundidad.** Un enlace necesita los enlaces de los prefijos más cortos; solo el orden en anchura los garantiza.
- **Volver a la raíz tras una coincidencia.** Así se pierden los solapamientos; el recorrido simplemente continúa.

## Cuándo usarlo

Úsalo para buscar a la vez un diccionario entero de palabras en un texto: filtrar palabras prohibidas, buscar firmas conocidas, etiquetar palabras clave.

## Temas relacionados

- [Trie (árbol de prefijos)](learn:trees/trie): el árbol sobre el que se construye este autómata.
- [Knuth-Morris-Pratt](learn:strings/kmp): la versión de los enlaces de fallo para un solo patrón.
- [Búsqueda en anchura (BFS)](learn:graphs/bfs): el orden en que se construyen los enlaces.
- [Rabin-Karp](learn:strings/rabin-karp): muchos patrones de una misma longitud, con hashes.
