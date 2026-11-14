## La idea

Un árbol de segmentos cambia una posición en `O(log n)`, pero sumar un valor a todo un rango posición a posición cuesta `O(n log n)`. La **propagación perezosa** hace que las actualizaciones de rangos también cuesten `O(log n)`, aplazando el trabajo hasta que hace falta.

Cada nodo guarda la **suma** de su rango y una **etiqueta**: una cantidad ya sumada a todo su rango pero que aún no se ha pasado a sus hijos. El arreglo empieza en ceros.

- **add l r v** baja como una suma de rango. Un nodo entero fuera de `[l, r]` se salta. Un nodo entero dentro se actualiza en el acto: su suma crece `v` por su longitud, su etiqueta crece `v`, y el recorrido se para ahí. Un nodo cubierto en parte primero **empuja** su etiqueta a los dos hijos (cada uno la suma a su etiqueta, y la etiqueta por su longitud a su suma) y la pone a 0; después el recorrido entra en los dos hijos y recalcula la suma del nodo con las suyas.
- **sum l r** es la suma de rango de siempre, salvo que empuja la etiqueta de un nodo cubierto en parte antes de bajar por él.

Así, las etiquetas solo bajan cuando una operación posterior necesita pasar por debajo de ellas.

### La entrada por defecto

n = 8. add 0 5 2 toma dos nodos enteros: etiqueta [0, 3] (su suma crece 8) y [4, 5] (crece 4), y la suma de la raíz pasa a 12. add 3 7 1 tiene que entrar en [0, 3], así que empuja esa etiqueta de 2 a [0, 1] y [2, 3]; dentro de [2, 3] vuelve a empujar, a [2, 2] y [3, 3]. Después [3, 3] recibe el +1 (suma 3) y [4, 7] se etiqueta con +1, y su suma crece 4 hasta 8. El arreglo es ahora 2 2 2 3 3 3 1 1. sum 2 6 lee [2, 3] directamente (5), empuja la etiqueta de [4, 7] para leer [4, 5] (6) y luego empuja la de [6, 7] para leer [6, 6] (1): 12. sum 0 0 empuja la etiqueta de [0, 1] y lee 2. La salida es 12, 2.

## Pasos

1. Para aplicar `v` a un nodo: suma `v` por su longitud a su suma y `v` a su etiqueta.
2. Para empujar un nodo: si su etiqueta no es 0, aplícala a los dos hijos y ponla a 0.
3. add l r v: sáltate los nodos de fuera, aplica `v` a los de dentro y, en cualquier otro nodo, empuja, entra en los dos hijos y recalcula su suma.
4. sum l r: da 0 fuera y la suma guardada dentro; en cualquier otro nodo, empuja y suma las respuestas de los dos hijos.

## Coste

| Operación | Coste |
|---|---|
| add sobre un rango | `O(log n)` |
| sum sobre un rango | `O(log n)` |
| Espacio extra | `O(n)`: sumas y etiquetas, `4n` casillas cada una |

## Errores comunes

- **Leer los hijos antes de empujar.** Un hijo bajo un nodo etiquetado no tiene en cuenta esa etiqueta; empuja siempre antes de bajar.
- **Olvidar la longitud.** Una etiqueta cuenta por posición. Al aplicarla, la suma de un nodo crece la etiqueta por el número de posiciones que cubre, no solo la etiqueta.
- **Saltarte el recálculo en add.** Tras cambiar los hijos, la suma del nodo tiene que rehacerse con ellos, o las sumas posteriores que se paren en este nodo saldrán mal.

## Cuándo usarlo

Usa propagación perezosa cuando cambian rangos enteros y siguen llegando preguntas sobre rangos: sumar a un tramo de días o mover todos los precios de un rango. Otros tipos de etiqueta (como "poner a x") funcionan igual una vez sabes cómo se combinan dos etiquetas y cómo cambia una etiqueta una suma.

## Temas relacionados

- [Árbol de segmentos](learn:trees/segment-tree): el árbol sobre el que se construye, con actualizaciones puntuales.
- [Árbol de Fenwick](learn:trees/fenwick-tree): una estructura más ligera para sumas prefijas e incrementos puntuales.
- [Sumas prefijas](learn:foundations/prefix-sums): sumas de rangos cuando el arreglo no cambia.
