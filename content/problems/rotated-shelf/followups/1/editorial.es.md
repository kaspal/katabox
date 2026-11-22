## Idea

Podrías repetir la búsqueda O(log n) del problema base para cada etiqueta, pero es más
sencillo deshacer el cambio una sola vez. Busca con búsqueda binaria el número más
pequeño: si `shelf[mid]` es mayor que el último número, el inicio está a la derecha de
`mid`; si no, está en `mid` o a su izquierda. Con el inicio `s`, el índice `k` de la lista
ordenada está en la posición `(k + s) mod n` de la estantería, así que una búsqueda
binaria normal sobre ese array ordenado virtual (o sobre una copia rotada) responde cada
etiqueta; devuelve `-1` si el número encontrado no es la etiqueta.

## Complejidad

O(n + q log n) en tiempo con una copia ordenada, u O(log n + q log n) con la vista
virtual; O(n) u O(1) de memoria extra además de la salida, donde `q` es el número de
etiquetas.
