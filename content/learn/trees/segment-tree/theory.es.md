## La idea

Un **árbol de segmentos** responde **sumas de rangos** y **actualizaciones puntuales** sobre un arreglo, ambas en `O(log n)`. Un arreglo normal actualiza rápido pero suma un rango despacio; las sumas prefijas hacen lo contrario. Un árbol de segmentos queda entre los dos.

El árbol vive en un arreglo de nodos. El nodo 1 cubre todo el rango `[0, n − 1]`, y un nodo `k` que cubre `[lo, hi]` tiene dos hijos: `2k` cubre `[lo, mid]` y `2k + 1` cubre `[mid + 1, hi]`, donde `mid` es `(lo + hi) / 2` redondeado hacia abajo. Un nodo que cubre una sola posición es una hoja y guarda ese valor; cualquier otro nodo guarda la suma de sus dos hijos. Construirlo toca cada nodo una vez, `O(n)`, y `4n` casillas siempre bastan.

- **sum l r** empieza en la raíz. Un nodo cuyo rango queda entero fuera de `[l, r]` da 0. Un nodo entero dentro da su suma guardada, sin bajar más. Cualquier otro nodo pregunta a sus dos hijos y suma sus respuestas. Solo unos pocos nodos por nivel quedan cubiertos en parte, así que el recorrido sigue siendo `O(log n)`.
- **update i v** baja desde la raíz hasta la hoja de la posición `i` (a la izquierda cuando `i ≤ mid`), le pone `v` y, a la vuelta, recalcula cada nodo del camino como la suma de sus hijos.

### La entrada por defecto

Valores 5 3 8 6 1 4. La raíz guarda 27. El nodo 2 cubre [0, 2] con 16 y el nodo 3 cubre [3, 5] con 11. Debajo, el nodo 4 = [0, 1] guarda 8, el nodo 5 = [2, 2] guarda 8, el nodo 6 = [3, 4] guarda 7 y el nodo 7 = [5, 5] guarda 4. sum 1 4 toma tres nodos enteros dentro, la hoja [1, 1] (3), el nodo 5 (8) y el nodo 6 (7), y se salta las hojas [0, 0] y [5, 5]: la respuesta es 18. update 2 0 pone el nodo 5 a 0 y luego recalcula el nodo 2 (ahora 8) y la raíz (ahora 19). sum 0 5 cubre todo el arreglo, así que la raíz responde sola: 19. La salida es 18, 19.

## Pasos

1. Construir: una hoja toma su valor del arreglo; un nodo interno toma la suma de sus dos hijos.
2. sum l r: si el rango de un nodo queda fuera de `[l, r]`, devuelve 0.
3. Si queda dentro de `[l, r]`, devuelve la suma del nodo.
4. Si no, devuelve la respuesta del hijo izquierdo más la del derecho.
5. update i v: baja hasta la hoja de `i`, ponle `v` y recalcula cada nodo a la vuelta.

## Coste

| Operación | Coste |
|---|---|
| Construir | `O(n)` |
| sum, update | `O(log n)` |
| Espacio extra | `O(n)`: `4n` casillas de nodos |

## Errores comunes

- **Pocas casillas.** `2n` casillas solo bastan cuando `n` es una potencia de dos; con cualquier otro `n` las últimas hojas caen fuera del final. Usa `4n`.
- **Límites mezclados.** Aquí todos los rangos incluyen sus dos extremos, así que la prueba de fuera es `r < lo` o `hi < l`. Mezclar un rango semiabierto pierde o cuenta dos veces un extremo.
- **Saltarte el recálculo.** Tras poner la hoja, la suma de cada antecesor queda desfasada hasta rehacerla con sus hijos.

## Cuándo usarlo

Usa un árbol de segmentos cuando se mezclan preguntas sobre rangos y actualizaciones. Sirve para cualquier combinación que se pueda repartir entre dos mitades, como el mínimo, el máximo o el máximo común divisor, no solo para sumas. Para sumas con incrementos puntuales, un árbol de Fenwick es más corto; para actualizar rangos enteros, añade propagación perezosa.

## Temas relacionados

- [Sumas prefijas](learn:foundations/prefix-sums): sumas de rangos rápidas cuando el arreglo no cambia.
- [Árbol de Fenwick](learn:trees/fenwick-tree): sumas prefijas y actualizaciones puntuales con menos código.
- [Propagación perezosa](learn:trees/lazy-propagation): el mismo árbol con actualizaciones de rangos enteros.
- [Montículo binario y cola de prioridad](learn:trees/binary-heap): otro árbol guardado en un arreglo con aritmética de índices.
