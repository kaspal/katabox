La búsqueda en anchura encuentra la ruta con menos carreteras, pero los peajes son distintos, así que menos carreteras no siempre es más barato. ¿De qué pueblo puedes estar seguro primero de su coste mínimo?
---
Todos los peajes son positivos. Entre los pueblos alcanzables, el de menor coste conocido ya no puede abaratarse: cualquier otro camino hasta él pasa por un pueblo que ya cuesta al menos lo mismo.
---
Algoritmo de Dijkstra: guarda `dist` (∞ salvo `dist[start] = 0`) y un montículo de mínimos de `(coste, pueblo)`. Saca el más barato; ignóralo si está desactualizado; si no, relaja cada carretera que sale de él, metiendo `(dist[t] + toll, otro)` cuando mejora al otro pueblo. Para al sacar `goal`.
