## Enfoque: las dos pasadas de Kosaraju

1. **Orden de finalización:** lanza un DFS sobre el grafo desde cada usuario sin visitar, añadiendo cada usuario a `order` cuando ha terminado con todos sus enlaces de salida.
2. **Pasada inversa:** construye el grafo invertido. Toma los usuarios desde el final de `order` (el que termina más tarde primero); cada uno aún sin asignar empieza un DFS sobre el grafo invertido que reúne exactamente su grupo. Anota el tamaño del grupo.
3. Ordena los tamaños.

**Por qué funciona:** el usuario que termina el último está en un grupo "fuente" (no le llegan enlaces desde otros grupos). En el grafo invertido ese grupo no tiene enlaces **de salida** hacia grupos sin asignar, así que el DFS inverso no puede salirse de él. Quitándolo, el argumento se repite.

Las dos pasadas de DFS usan pilas explícitas, así que las cadenas largas no chocan con los límites de recursión.

**Complejidad:** O(n + m) en tiempo y espacio. Comprobar por separado el alcance de cada usuario (un BFS por usuario y después comparar) es O(n·(n + m)).
