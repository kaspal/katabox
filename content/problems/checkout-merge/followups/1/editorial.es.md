## Idea

Haz la fusión de k colas del problema base y filtra sobre la marcha. Mete la cabeza de
cada cola no vacía en un montículo de mínimos ordenado por `(valor, índice de la cola)`.
Saca una y otra vez el nodo menor y mete su `next` si existe. Como la fusión saca los
valores en orden no decreciente, todas las copias de un número salen seguidas, así que
añade un nodo nuevo al resultado solo cuando el valor sea distinto del último añadido. Un
nodo cabecera ficticio simplifica las inserciones; devuelve su `next` (vacío si no había
clientes).

## Complejidad

O(N log k) en tiempo para `N` clientes en `k` colas, y O(k) de memoria extra para el
montículo, más la lista de salida. Reunir todos los números en un conjunto y ordenarlos
también sirve, en O(N log N).
